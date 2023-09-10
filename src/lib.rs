#![deny(clippy::all)]
#![warn(rustdoc::all)]

pub mod ast;
#[cfg(feature = "cli")]
pub mod cli;
pub mod error;
pub mod lexer;
pub mod macros;
pub mod parser;

pub use ast::{BinaryAction, BinaryNode, NodeBox, Number, PlainNode, UnaryAction, UnaryNode};
#[cfg(feature = "cli")]
pub use cli::CliParser;
pub use error::{LexError, ParseError, TilError};
pub use lexer::{Function, Lexer, Operator, Span, Token, TokenKind};
pub use parser::Parser;
