/*!
A simple binary expression tree, for parsing and preparing expressions which can be executed on dynamic contents.

An expression is built by calling the `push_operator`, `open_par`, `close_par` and `push_atom` functions.

It can then be evaluated with the `eval` function which takes as parameters

* a function which gives a value to an atom
* a function which, given an operator and one or two values, gives a new value
* a function deciding whether to shortcut

Normal evaluation order is left to right but is modified with parenthesis.

# Example : parsing and evaluating boolean expressions

Here we parse expressions like `"(A | B) & !(C | D | E)"` and evaluate them.

```
use bet::BeTree;

/// The operators in this example are AND, OR, and NOT operating on booleans.
/// `And` and `Or` are binary while `Not` is unary.
#[derive(Debug, Clone, Copy, PartialEq)]
enum BoolOperator {
    And,
    Or,
    Not,
}

/// simple but realistic example of an expression parsing.
///
/// You don't have to parse tokens in advance, you may accumulate
/// into atoms with `mutate_or_create_atom`.
///
/// For more reliable results on user inputs, you may want to check
/// the consistency (or use default operators) during parsing
/// with `accept_atom`, `accept_unary_operator`, etc.
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

/// evaluate the expression.
///
/// `trues` is the set of chars whose value is true.
///
/// If no operation is expected to fail, you may use
/// `eval` instead of `eval_faillible`, for a simpler
/// API.
fn eval(expr: &BeTree<BoolOperator, char>, trues: &[char]) -> bool {
    expr.eval_faillible(
        // the function evaluating leafs - here it's simple
        |c| Ok(trues.contains(c)),
        // the function applying an operator to one or two values
        |op, a, b| match (op, b) {
            (BoolOperator::And, Some(b)) => Ok(a & b),
            (BoolOperator::Or, Some(b)) => Ok(a | b),
            (BoolOperator::Not, None) => Ok(!a),
            _ => { Err("unexpected operation") }
        },
        // when to short-circuit. This is essential when leaf
        // evaluation is expensive or when the left part guards
        // for correctness of the right part evaluation
        |op, a| match (op, a) {
            (BoolOperator::And, false) => true,
            (BoolOperator::Or, true) => true,
            _ => false,
        },
    ).unwrap().unwrap()
}

// checking complex evaluations with T=true and F=false
assert_eq!(eval(&parse("!((T|F)&T)"), &['T']), false);
assert_eq!(eval(&parse("!(!((T|F)&(F|T)&T)) & !F & (T | (T|F))"), &['T']), true);

// we evaluate an expression with two different sets of values
let expr = parse("(A | B) & !(C | D | E)");
assert_eq!(eval(&expr, &['A', 'C', 'E']), false);
assert_eq!(eval(&expr, &['A', 'B']), true);

// Let's show the left to right evaluation order
// and importance of parenthesis
assert_eq!(eval(&parse("(A & B)|(C & D)"), &['A', 'B', 'C']), true);
assert_eq!(eval(&parse(" A & B | C & D "), &['A', 'B', 'C']), false);

```
*/


mod be_tree;

#[cfg(test)]
mod test_bool;
#[cfg(test)]
mod test_bool_faillible;

pub use {
    be_tree::BeTree,
};

