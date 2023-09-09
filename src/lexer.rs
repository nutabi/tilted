//! This modules implements a lexer, or tokeniser, for [`tilted`](crate).
//!
//! A lexer's job is to generate a stream of [`Token`]s from user input, which
//! is used by the [`Parser`] to generate an Abstract Syntax Tree.

use std::{fmt::Display, ops::Index, slice::SliceIndex};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{eof, token, LexError};

/// Special [`Result`] type for the lexer.
type Result<T> = std::result::Result<T, LexError>;

/// Lexer for [`tilted`](crate). It parses user input and return [`Token`]s.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Lexer {
    /// The original source code that is passed in.
    source_code: Box<str>,

    /// The index of the current character, i.e. the one that is parsed next.
    current_index: usize,
}

/// Part of the source code tokenised. Returned by a [`Lexer`].
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Token {
    /// Type of this [`Token`].
    pub kind: TokenKind,

    /// Location of this [`Token`].
    pub span: Span,
}

/// Type of a [`Token`], also containing the information associated.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TokenKind {
    /// End-of-file token. Note that the [`Span`] associated with EOF is
    /// out-of-bound, meaning if the span is used to look up source code, the
    /// slice will be zero-length.
    Eof,

    /// Integer, i.e. numbers without decimal places.
    Int(u64),

    /// Floating-point number, i.e. real numbers that are not integers.
    Flt(f64),

    /// Operator.
    Op(Operator),

    /// Function.
    Func(Function),

    /// Left parenthesis.
    LeftParen,

    /// Right parenthesis.
    RightParen,
}

/// Functions.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Function {
    /// Sine.
    Sin,

    /// Cosine.
    Cos,

    /// Tangent.
    Tan,

    /// Cosecant.
    Csc,

    /// Secant.
    Sec,

    /// Cotangent.
    Cot,

    /// Inverse sine.
    Asin,

    /// Inverse cosine.
    Acos,

    /// Inverse tangent.
    Atan,

    /// Inverse cosecant.
    Acsc,

    /// Inverse secant.
    Asec,

    /// Inverse cotangent.
    Acot,
}

/// Basic mathematical operators.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Operator {
    /// Operator `+`.
    Plus,

    /// Operator `-`
    Minus,

    /// Operator `*`.
    Star,

    /// Operator `/`.
    Slash,

    /// Operator `^`.
    Caret,
}

/// Spatial information of a [`Token`].
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Span {
    /// Index of the first character of this [`Span`].
    pub start_index: usize,

    /// Index of the last character of this [`Span`].
    pub end_index: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} at index {}", self.kind, self.span.start_index)
    }
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Plus,
            '-' => Self::Minus,
            '*' => Self::Star,
            '/' => Self::Slash,
            '^' => Self::Caret,

            // This also guards against attempts to add new operators
            // without implementing its conversion.
            _ => unreachable!("Unknown operator conversion"),
        }
    }
}

impl TryFrom<&str> for Function {
    type Error = ();
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "sin" => Ok(Self::Sin),
            "cos" => Ok(Self::Cos),
            "tan" => Ok(Self::Tan),
            "csc" => Ok(Self::Csc),
            "sec" => Ok(Self::Sec),
            "cot" => Ok(Self::Cot),
            "asin" => Ok(Self::Asin),
            "acos" => Ok(Self::Acos),
            "atan" => Ok(Self::Atan),
            "acsc" => Ok(Self::Acsc),
            "asec" => Ok(Self::Asec),
            "acot" => Ok(Self::Acot),
            _ => Err(()),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::Sin => write!(f, "Sin"),
            Function::Cos => write!(f, "Cos"),
            Function::Tan => write!(f, "Tan"),
            Function::Csc => write!(f, "Csc"),
            Function::Sec => write!(f, "Sec"),
            Function::Cot => write!(f, "Cot"),
            Function::Asin => write!(f, "Asin"),
            Function::Acos => write!(f, "Acos"),
            Function::Atan => write!(f, "Atan"),
            Function::Acsc => write!(f, "Acsc"),
            Function::Asec => write!(f, "Asec"),
            Function::Acot => write!(f, "Acot"),
        }
    }
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

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lex() {
            Ok(token) if token.kind != TokenKind::Eof => Some(token),
            _ => None,
        }
    }
}

impl Lexer {
    /// Creates a new [`Lexer`] from source code.
    #[allow(unused)]
    pub fn from_source_code<T: AsRef<str>>(source_code: T) -> Lexer {
        Lexer {
            source_code: Box::from(source_code.as_ref()),
            current_index: 0,
        }
    }

    /// Gets the next [`Token`] from source.
    pub fn lex(&mut self) -> Result<Token> {
        // Skip whitespaces.
        for c in self.source_code.chars().skip(self.current_index) {
            if c.is_whitespace() {
                self.current_index += 1;
            } else {
                break;
            }
        }

        // Check for EOF.
        if self.current_index >= self.source_code.len() {
            return Ok(eof!(self.current_index));
        }

        // Assign to handlers based on the next character.
        match self
            .source_code
            .chars()
            .nth(self.current_index)
            .ok_or(LexError::InternalError(
                "Unable to unwrap next character in source",
                self.current_index,
            ))? {
            // Numbers (integers and reals)
            // Can start with a dot or number
            '.' | '0'..='9' => self.handle_number(),

            // Operators.
            '+' | '-' | '*' | '/' | '^' => self.handle_operator(),

            // Parentheses.
            // These are short so they are handled in-place.
            '(' => {
                self.current_index += 1;
                Ok(token!(TokenKind::LeftParen, self.current_index - 1, 1))
            }
            ')' => {
                self.current_index += 1;
                Ok(token!(TokenKind::RightParen, self.current_index - 1, 1))
            }

            // Functions.
            c if c.is_ascii_alphabetic() => self.handle_function(),

            // Any other characters.
            c => Err(LexError::UnrecognisedCharacter(c, self.current_index)),
        }
    }

    pub fn handle_number(&mut self) -> Result<Token> {
        // Keep track of the original index for later.
        let original_index = self.current_index;

        // Tracker for decimal place.
        let mut seen_dot = false;

        // Reserve enough space for a 100-char string.
        // Most numbers (hopefully) are within this limit. However, we still
        // need to cover the potential cases of more than 100 digits.
        let mut result = String::with_capacity(100);
        for c in self.source_code[self.current_index..].chars() {
            match c {
                '.' => {
                    if !seen_dot {
                        // Dot (if not seen)
                        self.current_index += 1;
                        seen_dot = true;
                        result.push('.');
                    } else {
                        // Dot (if already seen)
                        return Err(LexError::UnrecognisedCharacter(c, self.current_index));
                    }
                }

                // Digit
                c if c.is_ascii_digit() => {
                    result.push(c);
                    self.current_index += 1;
                }

                // Anything else
                _ => break,
            }
        }

        // Convert string to integer or float based on seen_dot.
        if seen_dot {
            // Float
            let num = result
                .parse::<f64>()
                .map_err(|_| LexError::InternalError("Parse float failed", self.current_index))?;

            Ok(token!(TokenKind::Flt(num), original_index, result.len()))
        } else {
            // Integer
            let num = result
                .parse::<u64>()
                .map_err(|_| LexError::InternalError("Parse integer failed", self.current_index))?;

            Ok(token!(TokenKind::Int(num), original_index, result.len()))
        }
    }

    pub fn handle_operator(&mut self) -> Result<Token> {
        // Operator has only one char so it should be trivial.
        let op =
            self.source_code
                .chars()
                .nth(self.current_index)
                .ok_or(LexError::InternalError(
                    "Unable to unwrap operator",
                    self.current_index,
                ))?;

        // The parent match operator should have narrowed down the valid ones,
        // but I think it is still important to check here, just in case I mess
        // up somewhere else. Resources are cheap anyway :)
        match op {
            '+' | '-' | '*' | '/' | '^' => {
                self.current_index += 1;
                Ok(token!(TokenKind::Op(op.into()), self.current_index - 1, 1))
            }
            _ => Err(LexError::InternalError(
                "Invalid operator inside operator handler",
                self.current_index,
            )),
        }
    }

    pub fn handle_function(&mut self) -> Result<Token> {
        // Keep track of the original index for later.
        let original_index = self.current_index;

        // Trigos only contain letters.
        let name = self.source_code[self.current_index..]
            .chars()
            .take_while(|c| c.is_ascii_alphabetic())
            .collect::<String>();

        // Convert string to trigonometric function.
        let trigo = name
            .as_str()
            .try_into()
            .map_err(|_| LexError::UnrecognisedFunction(name.clone(), self.current_index))?;

        // Update current index.
        self.current_index += name.len();

        Ok(token!(TokenKind::Func(trigo), original_index, name.len()))
    }

    /// Reverts this [`Lexer`] to its original state.
    #[allow(unused)]
    pub fn reset(&mut self) {
        // Simply set the index to 0 to reset.
        self.current_index = 0;
    }
}
