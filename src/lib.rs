/*!
A simple binary expression tree, for parsing and preparing expressions which can be executed on dynamic contents.

An expression is built by calling the `push_operator`, `open_par`, `close_par` and `push_atom` functions.

It can then be evaluated with the `eval` function which takes as parameters

* a function which gives a value to an atom
* a function which, given an operator and two values, gives a new value

Normal evaluation order is left to right but is modified with parenthesis.

# Example : parsing and evaluating boolean expressions

Here we parse the `"(A | B) & !(C | D | E)"` expression
and evaluate it with different values of the `A` to `E` variables.

Then two other expressions are evaluated to display how parenthesis and evaluation work.

```
use bet::*;


/// The operators in this example are AND, OR, and NOT operating on booleans.
/// `And` and `Or` are binary while `Not` is unary.
/// Note that bet doesn't prevent an operator from being usable in both
/// unary and binary contexts.
#[derive(Debug, Clone, Copy, PartialEq)]
enum BoolOperator {
    And,
    Or,
    Not,
}
type BoolErr = &'static str;
impl BoolOperator {
    fn eval(self, a: bool, b: Option<bool>) -> Result<bool, BoolErr> {
        match (self, b) {
            (Self::And, Some(b)) => Ok(a & b),
            (Self::Or, Some(b)) => Ok(a | b),
            (Self::Not, None) => Ok(!a),
            _ => { Err("unexpected operation") }
        }
    }
}

fn parse(input: &str) -> BeTree<BoolOperator, char> {
    let mut expr = BeTree::new();
    for c in input.chars() {
        match c {
            '&' => expr.push_operator(BoolOperator::And),
            '|' => expr.push_operator(BoolOperator::Or),
            '!' => expr.push_operator(BoolOperator::Not),
            ' ' => {},
            '(' => expr.open_par(),
            ')' => expr.close_par(),
            _ => expr.push_atom(c),
        }
    }
    expr
}

let expr = parse("(A | B) & !(C | D | E)");
assert_eq!(
    expr.eval(
        |&c| Ok(c=='A'||c=='C'||c=='E'),
        |op, a, b| op.eval(a, b),
    ),
    Ok(Some(false)),
);
assert_eq!(
    expr.eval(
        |&c| Ok(c=='A'||c=='B'),
        |op, a, b| op.eval(a, b),
    ),
    Ok(Some(true)),
);

// Let's show the left to right evaluation order
// and importance of parenthesis
assert_eq!(
    parse("(A & B) | (C & D)").eval(
        |&c| Ok(c=='A' || c=='B' || c=='C'),
        |op, a, b| op.eval(a, b),
    ),
    Ok(Some(true)),
);
assert_eq!(
    parse("A & B | C & D").eval(
        |&c| Ok(c=='A' || c=='B' || c=='C'),
        |op, a, b| op.eval(a, b),
    ),
    Ok(Some(false)),
);

```
*/


mod be_tree;

#[cfg(test)]
mod test_bool;

pub use {
    be_tree::BeTree,
};

