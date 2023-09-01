//! This modules implements a parser for `cal`.
//!
//! A parser's job is to take in a stream of [`Token`] and produce an Abstract
//! Syntax Tree. The AST can be used to generate code or evaluate in the future.
use std::iter::Peekable;

use crate::{
    error::ParseError, BinaryAction, BinaryNode, Lexer, NodeBox, Number, Operator, PlainNode,
    TokenKind, UnaryAction, UnaryNode,
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

    /// Production:
    /// ```text
    /// expr = term ([+-] term)*
    /// ```
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
                Operator::Plus => BinaryAction::Add,
                Operator::Minus => BinaryAction::Sub,
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

    /// Production:
    /// ```text
    /// term = factor ([*/]? factor)*
    /// ```
    fn parse_term(&mut self) -> Result<NodeBox> {
        // Get the first factor.
        let mut factor = self.parse_factor()?;

        // Loop to get all factors.
        loop {
            // Get the operator.
            let operator = match self.lexer.peek() {
                Some(tok) => match tok.kind {
                    TokenKind::Op(op) => op,

                    // Check for implicit multiplication (left parenthesis)
                    TokenKind::LeftParen => {
                        let expr = self.parse_paren_expr()?;
                        let actor = BinaryAction::Mul;
                        factor = Box::new(BinaryNode::new(factor, actor, expr));

                        continue;
                    }

                    _ => return Ok(factor),
                },
                None => return Ok(factor),
            };

            // Match operator to actor.
            let actor = match operator {
                Operator::Star => BinaryAction::Mul,
                Operator::Slash => BinaryAction::Div,
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

    /// Production:
    /// ```text
    /// factor = [+-]* atomic
    /// ```
    fn parse_factor(&mut self) -> Result<NodeBox> {
        // Check for unary operator(s).
        let mut actor = UnaryAction::Iden;
        loop {
            let next_token = self.lexer.peek().ok_or(ParseError::UnexpectedEOF)?;
            match next_token.kind {
                TokenKind::Op(c) => match c {
                    Operator::Plus => (),
                    Operator::Minus => {
                        if actor == UnaryAction::Iden {
                            actor = UnaryAction::Neg;
                        } else {
                            actor = UnaryAction::Iden;
                        }
                    }

                    // Invalid unary operator.
                    _ => return Err(ParseError::InvalidUnaryOperator(*next_token)),
                },

                // No more unary operator.
                _ => break,
            };

            // Consume operator.
            self.lexer.next();
        }
        // Parse atomic.
        let operand = self.parse_atomic()?;

        if actor == UnaryAction::Iden {
            Ok(operand)
        } else {
            Ok(Box::new(UnaryNode::new(actor, operand)))
        }
    }

    /// Production:
    /// ```text
    /// atomic = Int | Flt | paren_expr
    /// ```
    fn parse_atomic(&mut self) -> Result<NodeBox> {
        // Match the next token.
        let next_token = *self.lexer.peek().ok_or(ParseError::UnexpectedEOF)?;
        let node = match next_token.kind {
            // Numbers (parse_numbers is merged here).
            TokenKind::Flt(f) => Box::new(PlainNode::new(Number::Flt(f))),
            TokenKind::Int(i) => Box::new(PlainNode::new(Number::Int(i as i128))),

            // Parenthesised expressions.
            TokenKind::LeftParen => self.parse_paren_expr()?,

            // Invalid unary operators, valid ones were handled up top.
            TokenKind::Op(_) => return Err(ParseError::InvalidUnaryOperator(next_token)),

            // Invalid:
            // - EOF: Impossible as next() is used.
            // - RightParen: Unmatched left parenthesis.
            _ => return Err(ParseError::MismatchRightParen(next_token.span.start_index)),
        };

        // Consume token.
        if next_token.kind != TokenKind::LeftParen {
            self.lexer.next();
        }

        Ok(node)
    }

    /// Production:
    /// ```text
    /// paren_expr = ( expr )
    /// ```
    fn parse_paren_expr(&mut self) -> Result<NodeBox> {
        // Expect a left parenthesis.
        let token = self.lexer.next().ok_or(ParseError::UnexpectedEOF)?;
        if token.kind != TokenKind::LeftParen {
            unreachable!()
        }

        // Parse expression.
        // Errors need to be return immediately as the lexer might be in an
        // unusable state.
        let expr = self.parse_expr()?;

        // Expect a right parenthesis.
        let token = self.lexer.next().ok_or(ParseError::UnexpectedEOF)?;
        if token.kind != TokenKind::RightParen {
            return Err(ParseError::MismatchRightParen(token.span.start_index));
        };

        Ok(expr)
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

    #[test]
    fn test_parser_int_unary_op() {
        let source = "7 * -5";
        let lexer = Lexer::from_source_code(source);
        let mut parser = Parser::new(lexer);
        let node = parser.parse();

        assert!(node.is_ok());

        let value = node.unwrap().evaluate();

        assert_eq!(value, Number::Int(-35));
    }

    #[test]
    fn test_parser_flt_unary_op() {
        let source = "7.0 * -5";
        let lexer = Lexer::from_source_code(source);
        let mut parser = Parser::new(lexer);
        let node = parser.parse();

        assert!(node.is_ok());

        let value = node.unwrap().evaluate();

        assert_eq!(value, Number::Flt(-35.0));
    }

    #[test]
    fn test_parser_impl_mul() {
        let source = "5(7 + 2)";
        let lexer = Lexer::from_source_code(source);
        let mut parser = Parser::new(lexer);
        let node = parser.parse();

        assert!(node.is_ok());

        let value = node.unwrap().evaluate();

        assert_eq!(value, Number::Int(45));
    }

    #[test]
    fn test_parser_many_unary_op() {
        let source = "--+5";
        let lexer = Lexer::from_source_code(source);
        let mut parser = Parser::new(lexer);
        let node = parser.parse();

        assert!(node.is_ok());

        let value = node.unwrap().evaluate();

        assert_eq!(value, Number::Int(5));
    }

    #[test]
    fn test_parser_weird_expr() {
        let source = "2*-(3*(1+-(2)))";
        let lexer = Lexer::from_source_code(source);
        let mut parser = Parser::new(lexer);
        let node = parser.parse();

        assert!(node.is_ok());

        let value = node.unwrap().evaluate();

        assert_eq!(value, Number::Int(6));
    }
}
