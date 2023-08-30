//! This modules implements a parser for `cal`.
//!
//! A parser's job is to take in a stream of [`Token`](crate::Token) and
//! produce an Abstract Syntax Tree. The AST can be used to generate code or
//! evaluate in the future.
