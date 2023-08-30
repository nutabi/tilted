//! This module implements the error types for `cal`.

use crate::Token;

#[derive(Debug, thiserror::Error)]
pub enum CalError {
    #[error("Lex error {0}")]
    LexError(#[from] LexError),

    #[error("Parse error {0}")]
    ParseError(#[from] ParseError),

    #[error("Unknown error: {0}")]
    Unknown(#[from] Box<dyn std::error::Error>),
}

#[derive(Debug, thiserror::Error)]
pub enum LexError {
    #[error("Unrecognised character '{0}' at index '{1}'")]
    UnrecognisedCharacter(char, usize),

    #[error("Internal error: {0}")]
    InternalError(&'static str),
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error(
        "Expected number, found '{:?}' at index {}",
        .0.kind,
        .0.span.start_index
    )]
    NumberExpected(Token),

    #[error(
        "Expected operator, found '{:?}' at index {}",
        .0.kind,
        .0.span.start_index
    )]
    OperatorExpected(Token),

    #[error(
        "Expected right parenthesis, found '{:?}' at index {}",
        .0.kind,
        .0.span.start_index
    )]
    RightParenExpected(Token),

    #[error(
        "Expected unary operator '+' or '-', found '{:?}' at index {}",
        .0.kind,
        .0.span.start_index
    )]
    InvalidUnaryOperator(Token),

    #[error("Unexpected right parenthesis found at index {0}")]
    MismatchRightParen(usize),

    #[error("Unknown error: {0}")]
    InternalError(&'static str),

    #[error("Expected something, found EOF")]
    UnexpectedEOF,
}
