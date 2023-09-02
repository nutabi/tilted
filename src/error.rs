//! This module implements the error types for [`tilted`](crate).
use crate::Token;
use std::{error::Error, fmt::Display};

/// Errors returned by [`tilted`](crate)
#[derive(Debug)]
pub enum TilError {
    /// Errors returned by [`Lexer`](crate::Lexer).
    Lex(LexError),

    /// Errors returned by [`Parser`](crate::Parser).
    Parse(ParseError),

    /// Errors from other sources.
    Unknown(Box<dyn Error>),
}

/// Errors returned by [`Lexer`](crate::Lexer).
#[derive(Debug, Clone)]
pub enum LexError {
    /// Character is not part of any [`Token`](crate::Token).
    UnrecognisedCharacter(char, usize),

    /// Errors caused by parsing valid but unexpected user input.
    InternalError(&'static str, usize),
}

/// Errors returned by [`Parser`](crate::Parser).
#[derive(Debug, Clone)]
pub enum ParseError {
    /// Expected a token, found end-of-file.
    UnexpectedEOF,

    /// Expected a number, found something else.
    NumberExpected(Token),

    /// Expected an operator, found something else.
    OperatorExpected(Token),

    /// Expected a right parenthesis, found something else.
    RightParenExpected(Token),

    /// Found an invalid unary operator.
    InvalidUnaryOperator(Token),

    /// Found a right parenthesis without a matching left parenthesis.
    MismatchRightParen(usize),

    /// Errors caused by parsing valid but unexpected user input.
    InternalError(&'static str),
}

impl Display for TilError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lex(e) => write!(f, "{}", e),
            Self::Parse(e) => write!(f, "{}", e),
            Self::Unknown(e) => write!(f, "{}", e),
        }
    }
}

impl Error for TilError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Lex(e) => Some(e),
            Self::Parse(e) => Some(e),
            Self::Unknown(e) => Some(e.as_ref()),
        }
    }
}

impl From<LexError> for TilError {
    fn from(value: LexError) -> Self {
        Self::Lex(value)
    }
}

impl From<ParseError> for TilError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnrecognisedCharacter(c, i) => {
                write!(f, "Unrecognised character '{}' at index {}", c, i)
            }
            Self::InternalError(e, i) => write!(f, "{} at index {}", e, i),
        }
    }
}

impl Error for LexError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEOF => write!(f, "Unexpected end-of-file"),
            Self::NumberExpected(t) => write!(f, "Expected a number, found {}", t),
            Self::OperatorExpected(t) => write!(f, "Expected an operator, found {}", t),
            Self::RightParenExpected(t) => write!(f, "Expected a right parenthesis, found {}", t),
            Self::InvalidUnaryOperator(t) => write!(f, "Found an invalid unary operator {}", t),
            Self::MismatchRightParen(i) => write!(
                f,
                "Found a right parenthesis without a matching left one at index {}",
                i
            ),
            Self::InternalError(s) => write!(f, "{}", s),
        }
    }
}

impl Error for ParseError {}
