use std::{fmt, ops::{self, ControlFlow}};

use num::{BigInt, BigRational, One, Signed, Zero};

use crate::{basic::Basic, expr::{Expr, ExprRef}, undefined::UNDEFINED, visitor::Visitor};

lazy_static::lazy_static! {
    pub static ref ZERO: Expr<Number> = Expr::new(Number::Integer(BigInt::zero()));
    pub static ref ONE: Expr<Number> = Expr::new(Number::Integer(BigInt::one()));
    pub static ref MINUS_ONE: Expr<Number> = Expr::new(Number::Integer(-BigInt::one()));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Number {
    Integer(BigInt),
    Rational(BigRational),
    Infinity(Sign),
}

impl Number {
    pub fn is_zero(&self) -> bool {
        match self {
            Number::Integer(i) => i.is_zero(),
            Number::Rational(r) => r.is_zero(),
            Number::Infinity(_) => false,
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            Number::Integer(i) => i.is_one(),
            Number::Rational(r) => r.is_one(),
            Number::Infinity(_) => false,
        }
    }

    pub fn is_minus_one(&self) -> bool {
        match self {
            Number::Integer(i) => i.magnitude().is_one() && i.is_negative(),
            Number::Rational(r) => r.numer().magnitude() == r.denom().magnitude() && r.is_negative(),
            Number::Infinity(_) => false,
        }
    }

    pub fn is_positive(&self) -> bool {
        match self {
            Number::Integer(i) => i.is_positive(),
            Number::Rational(r) => r.is_positive(),
            Number::Infinity(sign) => *sign == Sign::Plus,
        }
    }

    pub fn is_negative(&self) -> bool {
        match self {
            Number::Integer(i) => i.is_negative(),
            Number::Rational(r) => r.is_negative(),
            Number::Infinity(sign) => *sign == Sign::Minus,
        }
    }
}

impl Basic for Number {
    fn visit(self: ExprRef<Self>, visitor: &mut dyn Visitor) -> ControlFlow<()> {
        visitor.visit_number(self)
    }

    fn eq(&self, other: &dyn Basic) -> bool {
        if let Some(other) = other.downcast::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn cmp(&self, other: &dyn Basic) -> Option<std::cmp::Ordering> {
        other.downcast::<Self>().map(|other| Ord::cmp(self, other))
    }
}

impl ops::Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl ops::Add<&'_ Number> for Number {
    type Output = Number;

    fn add(self, rhs: &Self) -> Self::Output {
        &self + rhs
    }
}

impl ops::Add<&'_ Number> for &'_ Number {
    type Output = Number;

    fn add(self, rhs: &Number) -> Self::Output {
        match (&self, rhs) {
            // infinities
            (Number::Infinity(Sign::Plus), Number::Infinity(Sign::Plus)) => Number::Infinity(Sign::Plus),
            (Number::Infinity(Sign::Minus), Number::Infinity(Sign::Minus)) => Number::Infinity(Sign::Minus),
            (Number::Infinity(sign), _) | (_, Number::Infinity(sign)) => Number::Infinity(*sign),
            
            (Number::Integer(a), Number::Integer(b)) => Number::Integer(a + b),
            (Number::Integer(a), Number::Rational(b))
            | (Number::Rational(b), Number::Integer(a)) => Number::Rational(b + a),
            (Number::Rational(a), Number::Rational(b)) => Number::Rational(a + b)
        }
    }
}

impl ops::Mul<&'_ Number> for Number {
    type Output = Number;

    fn mul(self, rhs: &Number) -> Self::Output {
        &self * rhs
    }
}

impl ops::Mul<Number> for &'_ Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Self::Output {
        self * &rhs
    }
}

impl ops::Mul<&'_ Number> for &'_ Number {
    type Output = Number;

    fn mul(self, rhs: &Number) -> Self::Output {
        match (self, rhs) {
            (Number::Infinity(a), Number::Infinity(b)) => if a == b {
                Number::Infinity(Sign::Plus)
            } else {
                Number::Infinity(Sign::Minus)
            }
            (Number::Infinity(sign), _) | (_, Number::Infinity(sign)) => Number::Infinity(*sign),

            (Number::Integer(a), Number::Integer(b)) => Number::Integer(a * b),
            (Number::Integer(a), Number::Rational(b))
            | (Number::Rational(b), Number::Integer(a)) => Number::Rational(b * a),
            (Number::Rational(a), Number::Rational(b)) => Number::Rational(a * b)
        }
    }
}


impl From<u8> for Expr {
    fn from(n: u8) -> Self {
        Expr::new(Number::Integer(n.into()))
    }
}

impl From<u16> for Expr {
    fn from(n: u16) -> Self {
        Expr::new(Number::Integer(n.into()))
    }
}

impl From<u32> for Expr {
    fn from(n: u32) -> Self {
        Expr::new(Number::Integer(n.into()))
    }
}

impl From<u64> for Expr {
    fn from(n: u64) -> Self {
        Expr::new(Number::Integer(n.into()))
    }
}

impl From<u128> for Expr {
    fn from(n: u128) -> Self {
        Expr::new(Number::Integer(n.into()))
    }
}

impl From<i8> for Expr {
    fn from(n: i8) -> Self {
        Expr::new(Number::Integer(n.into()))
    }
}

impl From<i16> for Expr {
    fn from(n: i16) -> Self {
        Expr::new(Number::Integer(n.into()))
    }
}

impl From<i32> for Expr {
    fn from(n: i32) -> Self {
        Expr::new(Number::Integer(n.into()))
    }
}

impl From<i64> for Expr {
    fn from(n: i64) -> Self {
        Expr::new(Number::Integer(n.into()))
    }
}

impl From<i128> for Expr {
    fn from(n: i128) -> Self {
        Expr::new(Number::Integer(n.into()))
    }
}

impl From<f32> for Expr {
    fn from(f: f32) -> Self {
        BigRational::from_float(f)
            .map(|r| Expr::new(Number::Rational(r)) as Expr)
            .unwrap_or_else(|| {
                // the float was not finite
                if f.is_infinite() {
                    Expr::new(Number::Infinity(if f.is_sign_negative() {
                        Sign::Minus
                    } else {
                        Sign::Plus
                    }))
                } else {
                    UNDEFINED.clone()
                }
            })
    }
}
