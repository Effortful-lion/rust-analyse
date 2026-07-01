use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::grammar::{Grammar, ProblemInput, Production, Symbol};

pub fn load_problem(path: &Path) -> Result<ProblemInput, String> {
    let raw = fs::read_to_string(path).map_err(|error| format!("读取文件失败: {error}"))?;
    let sections = split_sections(&raw)?;

    let header_lines = collect_non_empty_lines(&sections[0]);
    let grammar_lines = collect_non_empty_lines(&sections[1]);
    let raw_inputs = collect_non_empty_lines(&sections[2]);

    let (start_symbol, terminals) = parse_header(&header_lines)?;
    let non_terminals = collect_non_terminals(&grammar_lines)?;
    validate_symbol_sets(&terminals, &non_terminals)?;
    if !non_terminals.contains(&start_symbol) {
        return Err(format!("开始符号未出现在产生式左部: {start_symbol}"));
    }

    let productions = build_productions(&grammar_lines, &non_terminals, &terminals)?;
    let productions_by_left = build_productions_by_left(&productions);
    let inputs = raw_inputs
        .into_iter()
        .map(|line| line.split_whitespace().map(str::to_string).collect())
        .collect();

    Ok(ProblemInput {
        grammar: Grammar {
            start_symbol,
            terminals,
            non_terminals,
            productions,
            productions_by_left,
        },
        inputs,
    })
}

fn parse_header(lines: &[&str]) -> Result<(String, BTreeSet<String>), String> {
    let mut start_symbol = None;
    let mut terminals = BTreeSet::new();

    for line in lines {
        if let Some(rest) = line.strip_prefix("%start ") {
            let value = rest.trim();
            if value.is_empty() {
                return Err("开始符号不能为空".to_string());
            }
            start_symbol = Some(value.to_string());
            continue;
        }

        if let Some(rest) = line.strip_prefix("%token ") {
            for token in rest.split_whitespace() {
                terminals.insert(token.to_string());
            }
            continue;
        }

        return Err(format!("无法识别的头部声明: {line}"));
    }

    let start_symbol = start_symbol.ok_or_else(|| "缺少开始符号声明".to_string())?;
    Ok((start_symbol, terminals))
}

fn split_sections(raw: &str) -> Result<Vec<Vec<&str>>, String> {
    let mut sections = vec![Vec::new()];

    for line in raw.lines() {
        if line.trim() == "%%" {
            sections.push(Vec::new());
            continue;
        }

        sections
            .last_mut()
            .expect("sections always has at least one entry")
            .push(line);
    }

    if sections.len() != 3 {
        return Err("输入文件必须包含两个 %% 分隔段".to_string());
    }

    Ok(sections)
}

fn collect_non_terminals(lines: &[&str]) -> Result<BTreeSet<String>, String> {
    let mut non_terminals = BTreeSet::new();

    for line in lines {
        let (left, _) = split_production(line)?;
        non_terminals.insert(left.to_string());
    }

    Ok(non_terminals)
}

fn validate_symbol_sets(
    terminals: &BTreeSet<String>,
    non_terminals: &BTreeSet<String>,
) -> Result<(), String> {
    let overlaps = terminals
        .intersection(non_terminals)
        .cloned()
        .collect::<Vec<_>>();

    if overlaps.is_empty() {
        return Ok(());
    }

    Err(format!(
        "terminal 与 non-terminal 不能重名: {}",
        overlaps.join(", ")
    ))
}

fn build_productions(
    lines: &[&str],
    non_terminals: &BTreeSet<String>,
    terminals: &BTreeSet<String>,
) -> Result<Vec<Production>, String> {
    let mut productions = Vec::new();

    for line in lines {
        let (left, right) = split_production(line)?;

        for branch in right.split('|') {
            let symbols = parse_right_side(branch.trim(), non_terminals, terminals)?;
            productions.push(Production {
                id: productions.len() + 1,
                left: left.to_string(),
                right: symbols,
            });
        }
    }

    Ok(productions)
}

fn build_productions_by_left(productions: &[Production]) -> BTreeMap<String, Vec<usize>> {
    let mut by_left = BTreeMap::new();

    for production in productions {
        by_left
            .entry(production.left.clone())
            .or_insert_with(Vec::new)
            .push(production.id);
    }

    by_left
}

fn split_production(line: &str) -> Result<(&str, &str), String> {
    let (left, right) = line
        .split_once("->")
        .ok_or_else(|| format!("产生式格式错误: {line}"))?;
    let left = left.trim();
    let right = right.trim();

    if left.is_empty() || right.is_empty() {
        return Err(format!("产生式格式错误: {line}"));
    }

    Ok((left, right))
}

fn parse_right_side(
    branch: &str,
    non_terminals: &BTreeSet<String>,
    terminals: &BTreeSet<String>,
) -> Result<Vec<Symbol>, String> {
    if branch.is_empty() {
        return Err("产生式右部不能为空".to_string());
    }

    if branch == "ε" {
        return Ok(vec![Symbol::Epsilon]);
    }

    let mut symbols = Vec::new();
    for part in branch.split_whitespace() {
        if part == "ε" {
            return Err("ε 必须单独作为一个产生式分支".to_string());
        }

        if non_terminals.contains(part) {
            symbols.push(Symbol::NonTerminal(part.to_string()));
            continue;
        }

        if terminals.contains(part) {
            symbols.push(Symbol::Terminal(part.to_string()));
            continue;
        }

        return Err(format!("符号未声明: {part}"));
    }

    Ok(symbols)
}

fn collect_non_empty_lines<'a>(lines: &'a [&'a str]) -> Vec<&'a str> {
    lines
        .iter()
        .copied()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
}
