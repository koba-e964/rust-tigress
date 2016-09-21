# Tigress [![Build Status](https://travis-ci.org/koba-e964/rust-tigress.svg?branch=master)](https://travis-ci.org/koba-e964/rust-tigress)
## Overview
This project contains a tiny interpreter/compiler for Tigress, a subset of Tiger language ([reference](http://www.cs.columbia.edu/~sedwards/classes/2002/w4115/tiger.pdf)).
The interpreter and the compiler are written in Rust, whereas in [the original project](http://github.com/koba-e964/tigress) they are written in Haskell.

## Dependency
This depends on nightly `rustc` and `cargo`. Nightly features are needed because of `rust-peg` and `docopt_macros`.
If you want to avoid using `docopt_macros` because it does not compile, you can work around it by
```
cargo run --features no-docopt-macros --no-default-features
```
. In this mode command-line arguments are simply ignored.
## Grammar
The grammar of Tigress is similar to Tiger, but there are some modifications. There are some features that are not supported in Tigress.

## functionality
|item|status (interpreter) |
|---|---|
| 1 Lexical Aspects | ok |
| 2.1 Lvalues | ok |
| 2.2 Return values | not supported |
| 2.3 Record and Array Literals | not supported |
| 2.4 Function Calls | not supported |
| 2.5 Operators | arithmetic and comparisons are supported |
| 2.6 Assignment | ok |
| 2.7 nil | ok |
| 2.8 Flow control | `for` is supported |
| 2.9 Let | not supported |
| 3 Declarations| not supported |
| 4 Standard Library | not supported |

