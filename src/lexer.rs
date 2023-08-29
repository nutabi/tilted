//! This modules implements a lexer, or tokeniser, for `cal`.
//! 
//! A lexer's job is to generate a stream of [`Token`]s from user input, which
//! is used by the [`Parser`] to generate an Abstract Syntax Tree.

use std::{ops::Index, slice::SliceIndex};

/// Lexer for `cal`. It parses user input and return [`Token`]s.
#[derive(Debug, Clone)]
pub struct Lexer {
    /// The original source code that is passed in.
    source_code: Box<str>,

    /// The index of the current character, i.e. the one that is parsed next.
    current_index: usize,
}

/// Part of the source code tokenised. Returned by a [`Lexer`].
#[derive(Debug, Clone, Copy)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

/// Type of a [`Token`], also containing the information associated.
#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Int(u128),
    Float(f64),
    Op(char),
    Var(char),
    LeftParen,
    RightParen,
}

/// Spatial information of a [`Token`].
#[derive(Debug, Clone, Copy)]
pub struct Span {
    /// Index of the first character of this [`Span`].
    start_index: usize,

    /// Index of the last character of this [`Span`].
    end_index: usize,
}

impl<Idx: SliceIndex<str>> Index<Idx> for Lexer {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.source_code[index]
    }
}

impl Index<Span> for Lexer {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.start_index..(index.end_index - 1)]
    }
}
