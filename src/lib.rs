/*! A simple binary expression tree,
for parsing and preparing expressions which can
be executed on dynamic contents.

# Example of parsing and evaluating a boolean expression

In this example bet helps parsing the `"(A | B) & (C | D | E)"` expression
and evaluate it with different values of the `A` to `E` variables.

Then two other expressions are evaluated to display how parenthesis and evaluation work.

```
use bet::*;

/// The operators in this example are AND and OR operating on booleans
#[derive(Debug, Clone, Copy, PartialEq)]
enum BoolOperator {
    And,
    Or,
}
impl Default for BoolOperator {
    fn default() -> Self {
        Self::And
    }
}
impl BoolOperator {
    fn eval(self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
        }
    }
}

fn parse(input: &str) -> BeTree<BoolOperator, char> {
    let mut expr = BeTree::new();
    for c in input.chars() {
        match c {
            '&' => expr.push_operator(BoolOperator::And),
            '|' => expr.push_operator(BoolOperator::Or),
            ' ' => {},
            '(' => expr.open_par(),
            ')' => expr.close_par(),
            _ => expr.push_atom(c),
        }
    }
    expr
}

let expr = parse("(A | B) & (C | D | E)");
assert_eq!(
    expr.eval(
        |&c| c=='A'||c=='C'||c=='E',
        |op, a, b| op.eval(a, b),
    ),
    Some(true),
);
assert_eq!(
    expr.eval(
        |&c| c=='A'||c=='B',
        |op, a, b| op.eval(a, b),
    ),
    Some(false),
);

// Let's show the left to right evaluation order
// and importance of parenthesis
assert_eq!(
    parse("(A & B) | (C & D)").eval(
        |&c| c=='A' || c=='B' || c=='C',
        |op, a, b| op.eval(a, b),
    ),
    Some(true),
);
assert_eq!(
    parse("A & B | C & D").eval(
        |&c| c=='A' || c=='B' || c=='C',
        |op, a, b| op.eval(a, b),
    ),
    Some(false),
);

```
*/


mod be_tree;

pub use {
    be_tree::BeTree,
};

