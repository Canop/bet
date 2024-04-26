[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/bet.svg
[l1]: https://crates.io/crates/bet

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/bet/badge.svg
[l3]: https://docs.rs/bet/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3

A library building and preparing expressions, for example boolean expressions such as `(A | B) & !(C | D | E)`,  which can be executed on dynamic contents.

An expression is built by sequentially pushing the parts: parenthesis, operators, atoms (the "variables").
You do that by calling the `push_operator`, `open_par`, `close_par` and `push_atom` functions, which build the tree for you.

It can then be evaluated with the `eval` function which takes as parameters

* a function which gives a value to an atom
* a function which, given an operator and one or two values, gives a new value
* a function deciding whether to short-circuit

Normal evaluation order is left to right but is modified with parenthesis.

**bet** is designed around separation of building, transformations, and evaluation, so that an expression can be efficiently applied on many inputs. **bet** is designed for very fast evaluation.

If you wonder whether bet could be applied to your problems, don't hesitate to [come and discuss](https://miaou.dystroy.org/3768).
## Known open-source usages

### dysk

**bet** is used in [dysk](https://dystroy.org/dysk) to filter filesystems.

Example: `dysk -f '(type=xfs & remote=no) | size > 5T'`.

Here, the atoms are `type=xfs`, `remote=no`, and `size > 5T`.

To parse such expression, the simplest solution is to first parse it with atoms being simple strings, then apply `try_map_atoms` on the tree to replace the strings with structs which can be efficiently evaluated on many entries.

Here's how it's done:

```rust
impl FromStr for Filter {
    type Err = ParseExprError;
    fn from_str(input: &str) -> Result<Self, ParseExprError> {

        // we start by reading the global structure
        let mut expr: BeTree<BoolOperator, String> = BeTree::new();
        for c in input.chars() {
            match c {
                '&' => expr.push_operator(BoolOperator::And),
                '|' => expr.push_operator(BoolOperator::Or),
                '!' => expr.push_operator(BoolOperator::Not),
                ' ' => {},
                '(' => expr.open_par(),
                ')' => expr.close_par(),
                _ => expr.mutate_or_create_atom(String::new).push(c),
            }
        }

        // then we parse each leaf
        let expr = expr.try_map_atoms(|raw| raw.parse())?;

        Ok(Self { expr })
    }
}
```

## broot

In [broot](https://dystroy.org/broot), **bet** enables composite queries on files.

For example, `!lock&(carg|c/carg/)` looks for files whose name or content contains "carg", but excluding files whose name contains "lock".

## rhit

**bet** is used in [rhit](https://dystroy.org/rhit) to filter log lines.

For example, with `rhit -p 'y & !( \d{4} | sp | bl )'`, you get stats on hits on paths containing "y" but neither a 4 digits number, "sp", nor "bl".

