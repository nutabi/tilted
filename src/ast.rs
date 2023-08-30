//! This modules implements a parser for `cal`.
//!
//! A parser's job is to take in a stream of [`Token`](crate::Token) and
//! produce an Abstract Syntax Tree. The AST can be used to generate code or
//! evaluate in the future.
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A convenience trait for [`f64`] and [`i128`] as they are the only two
/// number types allowed.
/// 
/// Note: Even though [`Token::Int`](crate::Token) stores [`u64`], the
/// [`Parser`](crate::Parser) will automatically converts it to [`i128`].
#[rustfmt::skip]
pub trait Number:
    Sized +
    Clone +
    Copy +
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

/// Convenience type aliase for a [`Node`] stored on the heap.
pub type NodeBox<T> = Box<dyn Node<T>>;

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
    left: NodeBox<T>,

    /// Action to be performed by this [`BinaryNode`].
    actor: BinaryAction,

    /// Right-hand side operand of this [`BinaryNode`].
    right: NodeBox<T>,
}

/// [`BinaryAction`] is an action done by a [`Node`] using one operand.
pub enum UnaryAction {
    Neg,
}

/// [`BinaryNode`] is a [`Node`] that performs an action on one operand.
pub struct UnaryNode<T: Number> {
    /// Action to be performed by this [`UnaryNode`].
    actor: UnaryAction,

    /// The sole operand of this [`UnaryNode`].
    operand: NodeBox<T>,
}

/// [`PlainNode`] simply stores the numbers without any action.
pub struct PlainNode<T: Number>(T);

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

impl<T: Number> BinaryNode<T> {
    /// Creates a new [`BinaryNode`].
    #[rustfmt::skip]
    pub fn new(
        left: NodeBox<T>,
        actor: BinaryAction,
        right: NodeBox<T>
    ) -> BinaryNode<T> {
        Self { left, actor, right }
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

impl<T: Number> UnaryNode<T> {
    /// Creates a new [`UnaryNode`].
    pub fn new(actor: UnaryAction, operand: NodeBox<T>) -> UnaryNode<T> {
        Self { actor, operand }
    }
}

impl<T: Number> Node<T> for PlainNode<T> {
    fn evaluate(&self) -> T {
        self.0
    }
}

impl<T: Number> PlainNode<T> {
    pub fn new(value: T) -> PlainNode<T> {
        Self(value)
    }
}
