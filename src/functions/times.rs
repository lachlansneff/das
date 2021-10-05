use std::{cmp::Ordering, ops::ControlFlow};

use crate::{basic::Basic, expr::Expr, visitor::Visitor};

/// This corresponds to `a * b * c * d`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn terms(&self) -> &[Expr] {
        &self.terms
    }
}

impl Basic for Times {
    fn visit(&self, visitor: &mut dyn Visitor) -> ControlFlow<()> {
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
