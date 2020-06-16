[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/bet.svg
[l1]: https://crates.io/crates/bet

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/bet/badge.svg
[l3]: https://docs.rs/bet/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3

A simple binary expression tree library, for parsing and preparing expressions which can be executed on dynamic contents.

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

// an expression which will be used with different sets of values
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
