# Tigress [![Build Status](https://github.com/koba-e964/rust-tigress/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/koba-e964/rust-tigress/actions/workflows/rust.yml?query=branch%3Amaster)
## Overview
This project contains a tiny interpreter/compiler for Tigress, a subset of Tiger language ([reference](http://www.cs.columbia.edu/~sedwards/classes/2002/w4115/tiger.pdf)).
The interpreter and the compiler are written in Rust, whereas in [the original project](http://github.com/koba-e964/tigress) they are written in Haskell.

## Dependency
Just run
```
cargo run
```
and everything will go well.
## Grammar
The grammar of Tigress is similar to Tiger, but there are some modifications. There are some features that are not supported in Tigress.

## Functionality
|item|status (interpreter) |
|---|---|
| 1 Lexical Aspects | ok |
| 2.1 Lvalues | ok |
| 2.2 Return values | not supported |
| 2.3 Record and Array Literals | not supported |
| 2.4 Function Calls | ok (environment for closure is not supported) |
| 2.5 Operators | ok |
| 2.6 Assignment | ok |
| 2.7 nil | ok |
| 2.8 Flow control | `for` and `do` are supported |
| 2.9 Let | var, function are supported |
| 3 Declarations | not supported |
| 4 Standard Library | not supported |
