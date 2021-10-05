use std::{cmp::Ordering, ops::ControlFlow};

use crate::{basic::Basic, expr::Expr, visitor::Visitor};

lazy_static::lazy_static! {
    pub static ref UNDEFINED: Expr = Expr::new(Undefined);
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Undefined;

impl Basic for Undefined {
    fn visit(&self, visitor: &mut dyn Visitor) -> ControlFlow<()> {
        visitor.visit_undefined(self)
    }

    fn eq(&self, other: &dyn Basic) -> bool {
        other.is::<Self>()
    }

    fn cmp(&self, other: &dyn Basic) -> Option<Ordering> {
        if let Some(other) = other.downcast::<Self>() {
            Some(Ord::cmp(self, other))
        } else {
            None
        }
    }
}