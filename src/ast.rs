//! This modules implements AST nodes and their evaluation mechanism.
//!
//! An Abstract Syntax Tree consists of [`Node`]s, which are built by a
//! [`Parser`](crate::Parser). AST can be evaluated or used to generate code.
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Internal representation of numbers.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Number {
    Int(i128),
    Flt(f64),
}

impl Add for Number {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Int(a) => match rhs {
                Self::Int(b) => Self::Int(a + b),
                Self::Flt(b) => Self::Flt((a as f64) + b),
            },
            Self::Flt(a) => match rhs {
                Self::Int(b) => Self::Flt(a + (b as f64)),
                Self::Flt(b) => Self::Flt(a + b),
            },
        }
    }
}

impl Sub for Number {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Int(a) => match rhs {
                Self::Int(b) => Self::Int(a - b),
                Self::Flt(b) => Self::Flt((a as f64) - b),
            },
            Self::Flt(a) => match rhs {
                Self::Int(b) => Self::Flt(a - (b as f64)),
                Self::Flt(b) => Self::Flt(a - b),
            },
        }
    }
}

impl Mul for Number {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Int(a) => match rhs {
                Self::Int(b) => Self::Int(a * b),
                Self::Flt(b) => Self::Flt((a as f64) * b),
            },
            Self::Flt(a) => match rhs {
                Self::Int(b) => Self::Flt(a * (b as f64)),
                Self::Flt(b) => Self::Flt(a * b),
            },
        }
    }
}

impl Div for Number {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::Int(a) => match rhs {
                Self::Int(b) => Self::Int(a / b),
                Self::Flt(b) => Self::Flt((a as f64) / b),
            },
            Self::Flt(a) => match rhs {
                Self::Int(b) => Self::Flt(a / (b as f64)),
                Self::Flt(b) => Self::Flt(a / b),
            },
        }
    }
}

impl Neg for Number {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Int(a) => Self::Int(-a),
            Self::Flt(a) => Self::Flt(-a),
        }
    }
}

/// [`Node`] provides a blanket trait for both [`BinaryNode`] and [`UnaryNode`].
pub trait Node {
    /// Finds the value of this [`Node`].
    fn evaluate(&self) -> Number;
}

/// Convenience type aliase for a [`Node`] stored on the heap.
pub type NodeBox = Box<dyn Node>;

/// [`BinaryAction`] is an action done by a [`Node`] using two operands.
pub enum BinaryAction {
    Add,
    Sub,
    Mul,
    Div,
}

/// [`BinaryNode`] is a [`Node`] that performs an action on two operands.
pub struct BinaryNode {
    /// Left-hand side operand (if any) of this [`BinaryNode`].
    left: NodeBox,

    /// Action to be performed by this [`BinaryNode`].
    actor: BinaryAction,

    /// Right-hand side operand of this [`BinaryNode`].
    right: NodeBox,
}

/// [`BinaryAction`] is an action done by a [`Node`] using one operand.
pub enum UnaryAction {
    Neg,
    Iden,
}

/// [`BinaryNode`] is a [`Node`] that performs an action on one operand.
pub struct UnaryNode {
    /// Action to be performed by this [`UnaryNode`].
    actor: UnaryAction,

    /// The sole operand of this [`UnaryNode`].
    operand: NodeBox,
}

/// [`PlainNode`] simply stores the numbers without any action.
pub struct PlainNode(Number);

impl BinaryAction {
    pub fn evaluate(&self, left: Number, right: Number) -> Number {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
        }
    }
}

impl Node for BinaryNode {
    fn evaluate(&self) -> Number {
        // Evaluate both sub-nodes.
        let left = self.left.evaluate();
        let right = self.right.evaluate();

        // Then evalute this node.
        self.actor.evaluate(left, right)
    }
}

impl BinaryNode {
    /// Creates a new [`BinaryNode`].
    #[rustfmt::skip]
    pub fn new(
        left: NodeBox,
        actor: BinaryAction,
        right: NodeBox
    ) -> BinaryNode {
        Self { left, actor, right }
    }
}

impl UnaryAction {
    pub fn evaluate(&self, operand: Number) -> Number {
        match self {
            Self::Neg => -operand,
            Self::Iden => operand,
        }
    }
}

impl Node for UnaryNode {
    fn evaluate(&self) -> Number {
        // Evaluate the operand.
        let operand = self.operand.evaluate();

        // Then evaluate this node.
        self.actor.evaluate(operand)
    }
}

impl UnaryNode {
    /// Creates a new [`UnaryNode`].
    pub fn new(actor: UnaryAction, operand: NodeBox) -> UnaryNode {
        Self { actor, operand }
    }
}

impl Node for PlainNode {
    fn evaluate(&self) -> Number {
        self.0
    }
}

impl PlainNode {
    pub fn new(value: Number) -> PlainNode {
        Self(value)
    }
}
