//! This modules implements a parser for `cal`.
//!
//! A parser's job is to take in a stream of [`Token`](crate::Token) and
//! produce an Abstract Syntax Tree. The AST can be used to generate code or
//! evaluate in the future.
use std::ops::{Add, Div, Neg, Mul, Sub};

/// A convenience trait for [`f64`] and [`i128`] as they are the only two
/// number types allowed.
/// 
/// Note: Even though [`Token::Int`](crate::Token) stores [`u64`], the
/// [`Parser`](crate::Parser) will automatically converts it to [`i128`].
#[rustfmt::skip]
pub trait Number:
    Sized +
    Add<Output = Self> +
    Sub<Output = Self> +
    Neg<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> { }

impl Number for i128 {}
impl Number for f64 {}

/// [`Node`] provides a blanket trait for both [`BinaryNode`] and [`UnaryNode`].
pub trait Node<T: Number> {
    /// Finds the value of this [`Node`].
    fn evaluate(&self) -> T;
}

/// [`BinaryAction`] is an action done by a [`Node`] using two operands.
pub enum BinaryAction {
    Add,
    Sub,
    Mul,
    Div,
}

/// [`BinaryNode`] is a [`Node`] that performs an action on two operands.
pub struct BinaryNode<T: Number> {
    /// Left-hand side operand (if any) of this [`BinaryNode`].
    left: Box<dyn Node<T>>,

    /// Action to be performed by this [`BinaryNode`].
    actor: BinaryAction,

    /// Right-hand side operand of this [`BinaryNode`].
    right: Box<dyn Node<T>>,
}

pub enum UnaryAction {
    Neg,
}

pub struct UnaryNode<T: Number> {
    /// Action to be performed by this [`UnaryNode`].
    actor: UnaryAction,

    /// The sole operand of this [`UnaryNode`].
    operand: Box<dyn Node<T>>,
}

impl BinaryAction {
    pub fn evaluate<T: Number>(&self, left: T, right: T) -> T {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
        }
    }
}

impl<T: Number> Node<T> for BinaryNode<T> {
    fn evaluate(&self) -> T {
        // Evaluate both sub-nodes.
        let left = self.left.evaluate();
        let right = self.right.evaluate();

        // Then evalute this node.
        self.actor.evaluate(left, right)
    }
}

impl UnaryAction {
    pub fn evaluate<T: Number>(&self, operand: T) -> T {
        match self {
            Self::Neg => -operand,
        }
    }
}

impl<T: Number> Node<T> for UnaryNode<T> {
    fn evaluate(&self) -> T {
        // Evaluate the operand.
        let operand = self.operand.evaluate();

        // Then evaluate this node.
        self.actor.evaluate(operand)
    }
}