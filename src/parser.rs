//! This modules implements a parser for `cal`.
//!
//! A parser's job is to take in a stream of [`Token`] and produce an Abstract
//! Syntax Tree. The AST can be used to generate code or evaluate in the future.

use std::marker::PhantomData;

use crate::{
    error::ParseError, BinaryAction, BinaryNode, Lexer, NodeBox, PlainNode, TokenKind, UnaryAction,
    UnaryNode,
};

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub struct Parser<T> {
    /// A [`Lexer`] used to retrieve tokens.
    lexer: Lexer,
    marker: PhantomData<T>,
}

// expr     = term [+|- term]*
// term     = factor [*|/ factor]*
// factor   = atomic
// atomic   = LeftParen expr RightParen
//          | Int
//          | Float
//          | +|- atomic

impl Parser<f64> {
    /// Creates a new [`Parser`] from a [`Lexer`].
    pub fn new(lexer: Lexer) -> Parser<f64> {
        Self {
            lexer,
            marker: PhantomData,
        }
    }

    pub fn parse(&mut self) -> Result<NodeBox<f64>> {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<NodeBox<f64>> {
        // Get the first term.
        let mut term = self.parse_term()?;

        // Loop to get all terms.
        loop {
            // Get the operator.
            let operator = match self.lexer.next() {
                Some(tok) => match tok.kind {
                    TokenKind::Op(op) => op,
                    _ => return Err(ParseError::OperatorExpected(tok)),
                },
                None => return Ok(term),
            };

            // Match operator to actor.
            let actor = match operator {
                '+' => BinaryAction::Add,
                '-' => BinaryAction::Sub,
                _ => {
                    return Err(ParseError::InternalError(
                        "Found operator but not '+' or '-'",
                    ))
                }
            };

            // Get the next term
            let next_term = self.parse_term()?;

            // Create a new node
            term = Box::new(BinaryNode::new(term, actor, next_term));
        }
    }

    fn parse_term(&mut self) -> Result<NodeBox<f64>> {
        // Get the first factor.
        let mut factor = self.parse_factor()?;

        // Loop to get all factors.
        // Loop to get all terms.
        loop {
            // Get the operator.
            let operator = match self.lexer.next() {
                Some(tok) => match tok.kind {
                    TokenKind::Op(op) => op,
                    _ => return Err(ParseError::OperatorExpected(tok)),
                },
                None => return Ok(factor),
            };

            // Match operator to actor.
            let actor = match operator {
                '*' => BinaryAction::Mul,
                '/' => BinaryAction::Div,
                _ => {
                    return Err(ParseError::InternalError(
                        "Found operator but not '*' or '/'",
                    ))
                }
            };

            // Get the next term
            let next_factor = self.parse_term()?;

            // Create a new node
            factor = Box::new(BinaryNode::new(factor, actor, next_factor));
        }
    }

    fn parse_factor(&mut self) -> Result<NodeBox<f64>> {
        self.parse_atomic()
    }

    fn parse_atomic(&mut self) -> Result<NodeBox<f64>> {
        // Match the next token.
        let next_token = self.lexer.next().ok_or(ParseError::UnexpectedEOF)?;
        let node = match next_token.kind {
            TokenKind::Float(f) => Box::new(PlainNode::new(f)),
            TokenKind::Int(i) => Box::new(PlainNode::new(i as f64)),
            TokenKind::LeftParen => {
                let expr = self.parse_expr()?;
                let next_token = self.lexer.next().ok_or(ParseError::UnexpectedEOF)?;
                match next_token.kind {
                    TokenKind::RightParen => expr,
                    _ => return Err(ParseError::RightParenExpected(next_token)),
                }
            }
            TokenKind::Op(c) => {
                let actor = match c {
                    '+' => UnaryAction::Iden,
                    '-' => UnaryAction::Neg,
                    _ => return Err(ParseError::InvalidUnaryOperator(next_token)),
                };
                let operand = self.parse_atomic()?;
                Box::new(UnaryNode::new(actor, operand))
            }
            _ => return Err(ParseError::MismatchRightParen(next_token.span.start_index)),
        };

        Ok(node)
    }
}
