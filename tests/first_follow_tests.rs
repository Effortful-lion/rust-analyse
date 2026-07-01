use std::collections::BTreeSet;
use std::path::Path;

use grammar_analyse::first_follow::{compute_first_sets, compute_follow_sets};
use grammar_analyse::loader::load_problem;

#[test]
fn compute_first_and_follow_for_expression_grammar() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");

    let first = compute_first_sets(&problem.grammar);
    let follow = compute_follow_sets(&problem.grammar, &first);

    assert_eq!(first["Expr"], set_of(&["(", "id"]));
    assert_eq!(first["ExprP"], set_of(&["+", "ε"]));
    assert_eq!(follow["Expr"], set_of(&["$", ")"]));
    assert_eq!(follow["Term"], set_of(&["$", ")", "+"]));
}

#[test]
fn first_of_expression_prime_contains_epsilon() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");

    let first = compute_first_sets(&problem.grammar);

    assert!(first["ExprP"].contains("ε"));
}

fn set_of(values: &[&str]) -> BTreeSet<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
