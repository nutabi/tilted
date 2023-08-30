#![deny(clippy::all)]
#![warn(rustdoc::all)]

pub mod ast;
pub mod error;
pub mod lexer;
pub mod macros;
pub mod parser;

pub use ast::{BinaryAction, BinaryNode, NodeBox, PlainNode, UnaryAction, UnaryNode};
pub use error::CalError;
pub use lexer::{Lexer, Span, Token, TokenKind};
