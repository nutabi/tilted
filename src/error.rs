//! This module implements the error types for `cal`.

#[derive(Debug, Clone)]
#[derive(thiserror::Error)]
pub enum CalError {
    #[error("Lex error {0}")]
    LexError(#[from] LexError)
}

#[derive(Debug, Clone)]
#[derive(thiserror::Error)]
pub enum LexError {
    #[error("Unrecognised character '{0}'")]
    UnrecognisedCharacter(char),
}