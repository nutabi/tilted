#![deny(clippy::all)]
#![warn(rustdoc::all)]

pub mod ast;
pub mod error;
pub mod lexer;
pub mod macros;
pub mod parser;

pub use ast::{BinaryAction, BinaryNode, NodeBox, Number, PlainNode, UnaryAction, UnaryNode};
pub use error::{CalError, LexError, ParseError};
pub use lexer::{Lexer, Operator, Span, Token, TokenKind};
pub use parser::Parser;
