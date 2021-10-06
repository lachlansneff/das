use std::{cmp::Ordering, ops::ControlFlow};

use crate::{Number, basic::Basic, expr::{Expr, ExprRef}, visitor::Visitor};

/// This corresponds to `a * b * c * d`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Times {
    terms: Vec<Expr>,
}

impl Times {
    pub fn new(terms: impl IntoIterator<Item = Expr>) -> Self {
        let mut terms: Vec<_> = terms.into_iter().collect();
        terms.sort_unstable();

        Self {
            terms,
        }
    }

    pub fn extend(&mut self, new: impl IntoIterator<Item = Expr>) {
        self.terms.extend(new);
        self.terms.sort_unstable();
    }

    pub fn terms(&self) -> &[Expr] {
        &self.terms
    }
}

pub fn times(lhs: ExprRef, rhs: ExprRef) -> Expr {
    match (lhs.downcast_exprref::<Times>(), rhs.downcast_exprref::<Times>()) {
        (Ok(times_rhs), Ok(times_lhs)) => {
            let mut times_rhs = times_rhs.into_expr();
            let mutable_times = Expr::make_mut(&mut times_rhs);
            mutable_times.extend(times_lhs.terms().iter().cloned());
            return times_rhs;
        },
        (Ok(times), Err(other))
        | (Err(other), Ok(times)) => {
            let mut times = times.into_expr();
            let mutable_times = Expr::make_mut(&mut times);
            mutable_times.extend([other.into_expr()]);
            return times;
        }
        _ => {},
    }

    if let (Some(lhs), Some(rhs)) = (lhs.downcast::<Number>(),  rhs.downcast::<Number>()) {
        Expr::new(lhs * rhs)
    } else {
        Expr::new(Times::new([lhs.into_expr(), rhs.into_expr()]))
    }
}

impl Basic for Times {
    fn visit(self: ExprRef<Self>, visitor: &mut dyn Visitor) -> ControlFlow<()> {
        visitor.visit_times(self)
    }

    fn eq(&self, other: &dyn Basic) -> bool {
        if let Some(other) = other.downcast::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn cmp(&self, other: &dyn Basic) -> Option<Ordering> {
        other.downcast::<Self>().map(|other| Ord::cmp(self, other))
    }
}
