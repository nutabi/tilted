#![deny(clippy::all)]
#![warn(rustdoc::all)]

pub mod error;
pub mod lexer;
pub mod macros;
pub mod parser;

pub use error::CalError;
pub use lexer::{Lexer, Span, Token, TokenKind};
