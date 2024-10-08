//! some tests with boolean expressions building and evaluating

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum BoolOperator {
    And,
    Or,
    Not,
}
impl BoolOperator {
    fn eval(self, a: bool, b: Option<bool>) -> bool {
        match (self, b) {
            (Self::And, Some(b)) => a & b,
            (Self::Or, Some(b)) => a | b,
            (Self::Not, None) => !a,
            _ => unreachable!(),
        }
    }
    /// tell whether we can skip evaluating the second operand
    fn short_circuit(self, a: bool) -> bool {
        matches!((self, a), (Self::And, false) | (Self::Or, true))
    }
}

fn check(input: &str, expected: bool) {
    let mut expr = BeTree::new();
    for c in input.chars() {
        match c {
            '&' => expr.push_operator(BoolOperator::And),
            '|' => expr.push_operator(BoolOperator::Or),
            '!' => expr.push_operator(BoolOperator::Not),
            ' ' => {}
            '(' => expr.open_par(),
            ')' => expr.close_par(),
            _ => expr.push_atom(c),
        }
    }
    let result = expr.eval(
        |&c| c == 'T',
        |op, a, b| op.eval(a, b),
        |op, &a| op.short_circuit(a),
    );
    assert_eq!(result, Some(expected));
}

#[test]
fn test_bool() {
    check("T", true);
    check("(((T)))", true);
    check("F", false);
    check("!T", false);
    check("!F", true);
    check("!!F", false);
    check("!!!F", true);
    check("F | T", true);
    check("F & T", false);
    check("F | !T", false);
    check("!F | !T", true);
    check("!(F & T)", true);
    check("!(T | T)", false);
    check("T | !(T | T)", true);
    check("T & (T & F)", false);
    check("!F & !(T & F & T)", true);
    check("!((T|F)&T)", false);
    check("!(!((T|F)&(F|T)&T)) & !F & (T | (T|F))", true);
    check("(T | F) & !T", false);
    check("!(T | F | T)", false);
    check("(T | F) & !(T | F | T)", false);
    check("F | !T | !(T & T | F)", false);
    check("(T & T) | (T & F)", true);
    check("T & T | T & F", false);
}

#[test]
fn issue_2() {
    check("F | F | F", false);
    check("F | F | F | F", false);
    check("F | T | F", true);
    check("F | T | F | F", true);
    check("F | F & F", false);
    check("F | F & F | F", false);
    check("F | T & F", false);
    check("F | T & F | F", false);
    check("F | F | T & F", false);
}
