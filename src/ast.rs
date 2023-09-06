//! This modules implements AST nodes and their evaluation mechanism.
//!
//! An Abstract Syntax Tree consists of [`Node`]s, which are built by a
//! [`Parser`](crate::Parser). AST can be evaluated or used to generate code.
use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Sub},
};

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

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Flt(n) => write!(f, "{}", n),
            Self::Int(n) => write!(f, "{}", n),
        }
    }
}

impl From<u8> for Number {
    fn from(n: u8) -> Self {
        Self::Int(n as i128)
    }
}

impl From<u16> for Number {
    fn from(n: u16) -> Self {
        Self::Int(n as i128)
    }
}

impl From<u32> for Number {
    fn from(n: u32) -> Self {
        Self::Int(n as i128)
    }
}

impl From<u64> for Number {
    fn from(n: u64) -> Self {
        Self::Int(n as i128)
    }
}

impl From<i8> for Number {
    fn from(n: i8) -> Self {
        Self::Int(n as i128)
    }
}

impl From<i16> for Number {
    fn from(n: i16) -> Self {
        Self::Int(n as i128)
    }
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Self::Int(n as i128)
    }
}

impl From<i64> for Number {
    fn from(n: i64) -> Self {
        Self::Int(n as i128)
    }
}

impl From<i128> for Number {
    fn from(n: i128) -> Self {
        Self::Int(n)
    }
}

impl From<f32> for Number {
    fn from(n: f32) -> Self {
        Self::Flt(n as f64)
    }
}

impl From<f64> for Number {
    fn from(n: f64) -> Self {
        Self::Flt(n)
    }
}

/// [`Node`] provides a blanket trait for both [`BinaryNode`] and [`UnaryNode`].
pub trait Node: Debug {
    /// Finds the value of this [`Node`].
    fn evaluate(&self) -> Number;
}

/// Convenience type aliase for a [`Node`] stored on the heap.
pub type NodeBox = Box<dyn Node>;

/// [`BinaryAction`] is an action done by a [`Node`] using two operands.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryAction {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

/// [`BinaryNode`] is a [`Node`] that performs an action on two operands.
#[derive(Debug)]
pub struct BinaryNode {
    /// Left-hand side operand (if any) of this [`BinaryNode`].
    left: NodeBox,

    /// Action to be performed by this [`BinaryNode`].
    actor: BinaryAction,

    /// Right-hand side operand of this [`BinaryNode`].
    right: NodeBox,
}

/// [`BinaryAction`] is an action done by a [`Node`] using one operand.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryAction {
    Neg,
    Iden,
}

/// [`BinaryNode`] is a [`Node`] that performs an action on one operand.
#[derive(Debug)]
pub struct UnaryNode {
    /// Action to be performed by this [`UnaryNode`].
    actor: UnaryAction,

    /// The sole operand of this [`UnaryNode`].
    operand: NodeBox,
}

/// [`PlainNode`] simply stores the numbers without any action.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlainNode(Number);

impl BinaryAction {
    pub fn evaluate(&self, left: Number, right: Number) -> Number {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
            Self::Pow => {
                // Integer base and exponent are kept as integer.
                if let Number::Int(n) = left {
                    if let Number::Int(m) = right {
                        if m >= 0 {
                            return Number::Int(n.pow(m as u32));
                        } else {
                            return Number::Flt((n as f64).powf(m as f64));
                        }
                    }
                }

                // Otherwise, both are converted to float.
                let left = match left {
                    Number::Int(n) => n as f64,
                    Number::Flt(n) => n,
                };
                let right = match right {
                    Number::Int(n) => n as f64,
                    Number::Flt(n) => n,
                };
                Number::Flt(left.powf(right))
            }
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
