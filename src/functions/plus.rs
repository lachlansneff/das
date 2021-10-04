use std::{any::Any, cmp::Ordering, sync::Arc};

use crate::{basic::Basic, symbol::Symbol, visitor::Visitor};



/// This corresponds to `a + b + c + d`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Plus {
    terms: Vec<Arc<dyn Basic>>,
}

impl Plus {
    pub fn new(terms: impl IntoIterator<Item = Arc<dyn Basic>>) -> Self {
        let mut terms: Vec<_> = terms.into_iter().collect();
        terms.sort_unstable();

        Self {
            terms,
        }
    }

    pub fn terms(&self) -> &[Arc<dyn Basic>] {
        &self.terms
    }
}

impl Basic for Plus {
    fn contains_symbol(&self, sym: &Symbol) -> bool {
        self.terms.iter().any(|x| x.contains_symbol(sym))
    }

    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_plus(self)
    }

    fn eq(&self, other: &dyn Basic) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn cmp(&self, other: &dyn Basic) -> Option<Ordering> {
        other.as_any().downcast_ref::<Self>().map(|other| Ord::cmp(self, other))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
