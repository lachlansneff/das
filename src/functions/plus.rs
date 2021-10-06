use std::{cmp::Ordering, ops::ControlFlow};

use crate::{Number, basic::Basic, expr::{Expr, ExprRef}, visitor::Visitor};



/// This corresponds to `a + b + c + d`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Plus {
    terms: Vec<Expr>,
}

impl Plus {
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

pub fn plus(lhs: ExprRef, rhs: ExprRef) -> Expr {
    match (lhs.downcast_exprref::<Plus>(), rhs.downcast_exprref::<Plus>()) {
        (Ok(plus_rhs), Ok(plus_lhs)) => {
            let mut plus_rhs = plus_rhs.into_expr();
            let mutable_plus = Expr::make_mut(&mut plus_rhs);
            mutable_plus.extend(plus_lhs.terms().iter().cloned());
            return plus_rhs;
        },
        (Ok(plus), Err(other))
        | (Err(other), Ok(plus)) => {
            let mut plus = plus.into_expr();
            let mutable_plus = Expr::make_mut(&mut plus);
            mutable_plus.extend([other.into_expr()]);
            return plus;
        }
        _ => {},
    }

    if let (Some(lhs), Some(rhs)) = (lhs.downcast::<Number>(),  rhs.downcast::<Number>()) {
        Expr::new(lhs + rhs)
    } else {
        Expr::new(Plus::new([lhs.into_expr(), rhs.into_expr()]))
    }
}

impl Basic for Plus {
    fn visit(self: ExprRef<Self>, visitor: &mut dyn Visitor) -> ControlFlow<()> {
        visitor.visit_plus(self)
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
