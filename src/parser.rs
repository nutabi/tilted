//! This modules implements a parser for [`tilted`](crate).
//!
//! A parser's job is to take in a stream of [`Token`] and produce an Abstract
//! Syntax Tree. The AST can be used to generate code or evaluate in the future.

use crate::{
    eof, BinaryAction, BinaryNode, Lexer, NodeBox, Number, Operator, ParseError, PlainNode,
    TilError, Token, TokenKind, UnaryAction, UnaryNode,
};

pub type Result<T> = std::result::Result<T, TilError>;

#[derive(Debug)]
pub struct Parser {
    /// A [`Lexer`] used to retrieve tokens.
    lexer: Lexer,

    /// The current token, allowing look-ahead once.
    current_token: Token,
}

impl Parser {
    /// Creates a new [`Parser`] from a [`Lexer`].
    pub fn from_lexer(lexer: Lexer) -> Parser {
        Self {
            lexer,
            current_token: eof!(0),
        }
    }

    /// Generates an AST.
    pub fn parse(&mut self) -> Result<NodeBox> {
        self.lex_and_store()?;

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
            let operator = match self.current_token.kind {
                TokenKind::Op(op) => op,
                _ => return Ok(term),
            };

            // Match operator to actor.
            let actor = match operator {
                Operator::Plus => BinaryAction::Add,
                Operator::Minus => BinaryAction::Sub,
                _ => return Ok(term),
            };

            // Next.
            self.lex_and_store()?;

            // Get the next term.
            let next_term = self.parse_term()?;

            // Create a new node.
            term = Box::new(BinaryNode::new(term, actor, next_term));
        }
    }

    /// Production:
    /// ```text
    /// term = factor ([*/] factor)*
    /// ```
    fn parse_term(&mut self) -> Result<NodeBox> {
        // Get the first factor.
        let mut factor = self.parse_factor()?;

        // Loop to get all factors.
        loop {
            // Get the actor.
            let actor = match self.current_token.kind {
                TokenKind::Op(op) => {
                    let a = match op {
                        Operator::Star => BinaryAction::Mul,
                        Operator::Slash => BinaryAction::Div,
                        _ => return Ok(factor),
                    };

                    // Consume operator.
                    self.lex_and_store()?;

                    a
                }

                // Check for implicit multiplication.
                // 1. Left parenthesis.
                TokenKind::LeftParen => BinaryAction::Mul,
                // 2. Function.
                TokenKind::Func(_) => BinaryAction::Mul,

                _ => return Ok(factor),
            };

            // Get the next factor.
            let next_factor = self.parse_factor()?;

            // Create a new node.
            factor = Box::new(BinaryNode::new(factor, actor, next_factor));
        }
    }

    /// Production:
    /// ```text
    /// factor = [+-]* pow
    /// ```
    fn parse_factor(&mut self) -> Result<NodeBox> {
        // Check for unary operator(s).
        let mut actor = UnaryAction::Iden;
        loop {
            match self.current_token.kind {
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
                    _ => return Err(ParseError::InvalidUnaryOperator(self.current_token).into()),
                },

                TokenKind::Eof => return Err(ParseError::UnexpectedEOF.into()),

                // No more unary operator.
                _ => break,
            };

            // Consume operator.
            self.lex_and_store()?;
        }

        // Parse atomic.
        let operand = self.parse_pow()?;

        if actor == UnaryAction::Iden {
            Ok(operand)
        } else {
            Ok(Box::new(UnaryNode::new(actor, operand)))
        }
    }

    /// Production:
    /// ```text
    /// pow = atomic (^ atomic)?
    /// ```
    fn parse_pow(&mut self) -> Result<NodeBox> {
        // Parse base.
        let base = self.parse_atomic()?;

        // Check for exponentiation.
        if self.current_token.kind != TokenKind::Op(Operator::Caret) {
            return Ok(base);
        }

        // Consume operator.
        self.lex_and_store()?;

        // Parse exponent.
        let exponent = self.parse_atomic()?;

        // Create a new node.
        Ok(Box::new(BinaryNode::new(base, BinaryAction::Pow, exponent)))
    }

    /// Production:
    /// ```text
    /// atomic = Int | Flt | paren_expr | Func paren_expr
    /// ```
    fn parse_atomic(&mut self) -> Result<NodeBox> {
        // Match the next token.
        let node: NodeBox = match self.current_token.kind {
            // Numbers (parse_numbers is merged here).
            TokenKind::Flt(f) => Box::new(PlainNode::new(Number::Flt(f))),
            TokenKind::Int(i) => Box::new(PlainNode::new(Number::Int(i as i128))),

            // Parenthesised expressions.
            // Return immediately to avoid consumption of current token.
            TokenKind::LeftParen => return self.parse_paren_expr(),

            // Functions.
            TokenKind::Func(func) => {
                // Consume function.
                self.lex_and_store()?;

                // Parse parenthesised expression.
                let expr = self.parse_paren_expr()?;

                // Create a new node.
                Box::new(UnaryNode::new(UnaryAction::Func(func), expr))
            }

            // Invalid unary operators, valid ones were handled up top.
            TokenKind::Op(_) => {
                return Err(ParseError::InvalidUnaryOperator(self.current_token).into())
            }

            // Catch all EOF.
            TokenKind::Eof => return Err(ParseError::UnexpectedEOF.into()),

            // Invalid:
            // RightParen: Unmatched left parenthesis.
            _ => {
                return Err(
                    ParseError::MismatchRightParen(self.current_token.span.start_index).into(),
                )
            }
        };

        // Consume token.
        self.lex_and_store()?;

        Ok(node)
    }

    /// Production:
    /// ```text
    /// paren_expr = LeftParen expr RightParen
    /// ```
    fn parse_paren_expr(&mut self) -> Result<NodeBox> {
        // Expect a left parenthesis.
        if self.current_token.kind != TokenKind::LeftParen {
            unreachable!()
        }

        // Consume left parenthesis.
        self.lex_and_store()?;

        // Parse expression.
        // Errors need to be return immediately as the lexer might be in an
        // unusable state.
        let expr = self.parse_expr()?;

        // Expect a right parenthesis.
        if self.current_token.kind != TokenKind::RightParen {
            return Err(ParseError::RightParenExpected(self.current_token).into());
        };

        // Consume right parenthesis.
        self.lex_and_store()?;

        Ok(expr)
    }

    fn lex_and_store(&mut self) -> Result<Token> {
        let token = self.lexer.lex()?;
        self.current_token = token;
        Ok(token)
    }
}
