//! This modules implements a parser for `cal`.
//!
//! A parser's job is to take in a stream of [`Token`] and produce an Abstract
//! Syntax Tree. The AST can be used to generate code or evaluate in the future.
use std::iter::Peekable;

use crate::{
    error::ParseError, BinaryAction, BinaryNode, Lexer, NodeBox, Number, PlainNode, TokenKind,
    UnaryAction, UnaryNode,
};

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub struct Parser {
    /// A [`Lexer`] used to retrieve tokens.
    lexer: Peekable<Lexer>,
}

impl Parser {
    /// Creates a new [`Parser`] from a [`Lexer`].
    pub fn new(lexer: Lexer) -> Parser {
        Self {
            lexer: lexer.peekable(),
        }
    }

    /// Generates an AST.
    pub fn parse(&mut self) -> Result<NodeBox> {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<NodeBox> {
        // Get the first term.
        let mut term = self.parse_term()?;

        // Loop to get all terms.
        loop {
            // Get the operator.
            let operator = match self.lexer.peek() {
                Some(tok) => match tok.kind {
                    TokenKind::Op(op) => op,
                    _ => return Ok(term),
                },
                None => return Ok(term),
            };

            // Match operator to actor.
            let actor = match operator {
                '+' => BinaryAction::Add,
                '-' => BinaryAction::Sub,
                _ => return Ok(term),
            };

            // Consume operator.
            self.lexer.next();

            // Get the next term.
            let next_term = self.parse_term()?;

            // Create a new node.
            term = Box::new(BinaryNode::new(term, actor, next_term));
        }
    }

    fn parse_term(&mut self) -> Result<NodeBox> {
        // Get the first factor.
        let mut factor = self.parse_factor()?;

        // Loop to get all factors.
        // Loop to get all terms.
        loop {
            // Get the operator.
            let operator = match self.lexer.peek() {
                Some(tok) => match tok.kind {
                    TokenKind::Op(op) => op,
                    _ => return Ok(factor),
                },
                None => return Ok(factor),
            };

            // Match operator to actor.
            let actor = match operator {
                '*' => BinaryAction::Mul,
                '/' => BinaryAction::Div,
                _ => return Ok(factor),
            };

            // Consume operator.
            self.lexer.next();

            // Get the next factor.
            let next_factor = self.parse_factor()?;

            // Create a new node.
            factor = Box::new(BinaryNode::new(factor, actor, next_factor));
        }
    }

    fn parse_factor(&mut self) -> Result<NodeBox> {
        self.parse_atomic()
    }

    fn parse_atomic(&mut self) -> Result<NodeBox> {
        // Match the next token.
        let next_token = self.lexer.next().ok_or(ParseError::UnexpectedEOF)?;
        let node = match next_token.kind {
            // Numbers
            TokenKind::Flt(f) => Box::new(PlainNode::new(Number::Flt(f))),
            TokenKind::Int(i) => Box::new(PlainNode::new(Number::Int(i as i128))),

            // Parenthesised expressions.
            TokenKind::LeftParen => {
                let expr = self.parse_expr()?;
                let next_token = self.lexer.next().ok_or(ParseError::UnexpectedEOF)?;
                match next_token.kind {
                    TokenKind::RightParen => expr,
                    _ => return Err(ParseError::RightParenExpected(next_token)),
                }
            }

            // Unary operators.
            TokenKind::Op(c) => {
                let actor = match c {
                    '+' => UnaryAction::Iden,
                    '-' => UnaryAction::Neg,
                    _ => return Err(ParseError::InvalidUnaryOperator(next_token)),
                };
                let operand = self.parse_atomic()?;
                Box::new(UnaryNode::new(actor, operand))
            }

            // Invalid:
            // - EOF: Impossible as next() is used.
            // - RightParen: Unmatched left parenthesis.
            _ => return Err(ParseError::MismatchRightParen(next_token.span.start_index)),
        };

        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_int() {
        let source = "7 + 6 * 2 - 4 * (8 + 3)";
        let lexer = Lexer::from_source_code(source);
        let mut parser = Parser::new(lexer);
        let node = parser.parse();

        assert!(node.is_ok());

        let value = node.unwrap().evaluate();

        assert_eq!(value, Number::Int(-25));
    }

    #[test]
    fn test_parser_flt() {
        let source = "7.0 + 6 * 2 - 4 * (8 + 3)";
        let lexer = Lexer::from_source_code(source);
        let mut parser = Parser::new(lexer);
        let node = parser.parse();

        assert!(node.is_ok());

        let value = node.unwrap().evaluate();

        assert_eq!(value, Number::Flt(-25.0));
    }
}
