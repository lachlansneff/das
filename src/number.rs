use std::{any::Any, ops, sync::Arc};

use num::{BigInt, BigRational, One, Signed, Zero};

use crate::{basic::Basic, symbol::{Symbol}, visitor::Visitor};

lazy_static::lazy_static! {
    pub static ref ZERO: Arc<dyn Basic> = Arc::new(Number::Integer(BigInt::zero()));
    pub static ref ONE: Arc<dyn Basic> = Arc::new(Number::Integer(BigInt::one()));
    pub static ref MINUS_ONE: Arc<dyn Basic> = Arc::new(Number::Integer(-BigInt::one()));
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
    fn contains_symbol(&self, _sym: &Symbol) -> bool {
        false
    }

    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_number(self);
    }

    fn eq(&self, other: &dyn Basic) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn cmp(&self, other: &dyn Basic) -> Option<std::cmp::Ordering> {
        other.as_any().downcast_ref::<Self>().map(|other| Ord::cmp(self, other))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ops::Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // infinities
            (Number::Infinity(Sign::Plus), Number::Infinity(Sign::Plus)) => Number::Infinity(Sign::Plus),
            (Number::Infinity(Sign::Minus), Number::Infinity(Sign::Minus)) => Number::Infinity(Sign::Minus),
            (Number::Infinity(sign), _) | (_, Number::Infinity(sign)) => Number::Infinity(sign),
            
            (Number::Integer(a), Number::Integer(b)) => Number::Integer(a + b),
            (Number::Integer(a), Number::Rational(b))
            | (Number::Rational(b), Number::Integer(a)) => Number::Rational(b + a),
            (Number::Rational(a), Number::Rational(b)) => Number::Rational(a + b)
        }
    }
}

impl ops::Add<&'_ Number> for Number {
    type Output = Number;

    fn add(self, rhs: &Self) -> Self::Output {
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

impl ops::Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Infinity(a), Number::Infinity(b)) => if a == b {
                Number::Infinity(Sign::Plus)
            } else {
                Number::Infinity(Sign::Minus)
            }
            (Number::Infinity(sign), _) | (_, Number::Infinity(sign)) => Number::Infinity(sign),

            (Number::Integer(a), Number::Integer(b)) => Number::Integer(a * b),
            (Number::Integer(a), Number::Rational(b))
            | (Number::Rational(b), Number::Integer(a)) => Number::Rational(b * a),
            (Number::Rational(a), Number::Rational(b)) => Number::Rational(a * b)
        }
    }
}