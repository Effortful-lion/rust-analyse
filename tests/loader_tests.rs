use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs};

use grammar_analyse::grammar::Symbol;
use grammar_analyse::loader::load_problem;

#[test]
fn load_expression_grammar_file() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");

    assert_eq!(problem.grammar.start_symbol, "Expr");
    assert!(problem.grammar.terminals.contains("+"));
    assert_eq!(problem.grammar.productions.len(), 8);
    assert_eq!(problem.inputs.len(), 3);
    assert_eq!(problem.inputs[0], vec!["id", "+", "id", "*", "id"]);
    assert_eq!(
        problem.grammar.productions[0].right,
        vec![
            Symbol::NonTerminal("Term".to_string()),
            Symbol::NonTerminal("ExprP".to_string()),
        ]
    );
}

#[test]
fn reject_conflict_fixture_with_unknown_start_symbol() {
    let error = load_problem(Path::new("tests/fixtures/conflict_grammar.txt"))
        .err()
        .expect("expected error");

    assert!(error.contains("开始符号未出现在产生式左部"));
}

#[test]
fn load_problem_accepts_crlf_and_blank_lines_around_section_delimiters() {
    let path = write_temp_problem(
        "loader-crlf",
        "%start Expr\r\n%token id +\r\n\r\n  %%  \r\n\r\nExpr -> id\r\n\r\n%%\r\n\r\nid\r\n",
    );

    let problem = load_problem(&path).expect("load crlf fixture");

    assert_eq!(problem.grammar.start_symbol, "Expr");
    assert_eq!(problem.grammar.productions.len(), 1);
    assert_eq!(problem.inputs, vec![vec!["id".to_string()]]);
}

#[test]
fn reject_grammar_when_terminal_and_non_terminal_share_the_same_name() {
    let path = write_temp_problem(
        "loader-name-conflict",
        "%start Expr\n%token Expr id\n%%\nExpr -> id\n%%\nid\n",
    );

    let error = load_problem(&path).err().expect("expected overlap error");

    assert!(error.contains("terminal"));
    assert!(error.contains("non-terminal"));
    assert!(error.contains("Expr"));
}

fn write_temp_problem(prefix: &str, contents: &str) -> std::path::PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock before unix epoch")
        .as_nanos();
    let path = env::temp_dir().join(format!(
        "{prefix}-{}-{unique}.txt",
        std::process::id()
    ));
    fs::write(&path, contents).expect("write temp problem");
    path
}
