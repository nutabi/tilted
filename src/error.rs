//! This module implements the error types for `cal`.

#[derive(Debug, thiserror::Error)]
pub enum CalError {
    #[error("Lex error {0}")]
    LexError(#[from] LexError),

    #[error("Unknown error {0}")]
    Unknown(#[from] Box<dyn std::error::Error>),
}

#[derive(Debug, thiserror::Error)]
pub enum LexError {
    #[error("Unrecognised character '{0}'")]
    UnrecognisedCharacter(char),

    #[error("Unknown error: '{0}'")]
    UnknownError(&'static str),
}
