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

An expression is built by calling the `push_operator`, `open_par`, `close_par` and `push_atom` functions.

It can then be evaluated with the `eval` function which takes as parameters

* a function which gives a value to an atom
* a function which, given an operator and one or two values, gives a new value
* a function deciding whether to short-circuit

Normal evaluation order is left to right but is modified with parenthesis.

**bet** is designed around separation of building, transformations, and evaluation, so that an expression can be efficiently applied on many inputs. **bet** is designed for very fast evaluation.

**bet** is used in [broot](https://dystroy.org/broot) to let users type composite queries on files.

**bet** is used in [rhit](https://github.com/Canop/rhit) to filter log lines.

Limits:
* no operator precedence: evaluation is left to right when there's no parenthesis

**Usage and documentation: [docs.rs/bet](https://docs.rs/bet/)**

If you wonder whether bet could be applied to your problems, don't hesitate to [come and discuss](https://miaou.dystroy.org/3768). If you know a documented crate with overlapping use cases, tell me too so that I may list it here as alternative.
