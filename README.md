# Tigress
## Overview
This project contains a tiny interpreter/compiler for Tigress, a subset of Tiger language ([reference](http://www.cs.columbia.edu/~sedwards/classes/2002/w4115/tiger.pdf)).
The interpreter and the compiler are written in Rust, whereas in [the original project](http://github.com/koba-e964/tigress) they are written in Haskell.

## Dependency
This depends on nightly `rustc` and `cargo`. Nightly features are needed because of `rust-peg`.

## Grammar
The grammar of Tigress is similar to Tiger, but there are some modifications. The major difference between them is the array creation. While we create an array by `type-id [ expr ] of expr` in Tiger, we create one by `new type-id [ expr ] OF expr` (for the simplicity of parser) in Tigress. Besides, there are some features that are not supported in Tigress.

## functionality
|item|status (interpreter) |
|---|---|
| 1 Lexical Aspects | `if`, `while`, `let`, `struct` are not supported |
| 2.1 Lvalues | not supported |
| 2.2 Return values | not supported |
| 2.3 Record and Array Literals | not supported |
| 2.4 Function Calls | not supported |
| 2.5 Operators | not supported |
| 2.6 Assignment | not supported |
| 2.7 nil | not supported |
| 2.8 Flow control | not supported |
| 2.9 Let | not supported |
| 3 Declarations| not supported |
| 4 Standard Library | not supported |

