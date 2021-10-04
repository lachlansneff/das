use std::{cmp::Ordering, sync::Arc};

use crate::{basic::Basic, symbol::Symbol, visitor::Visitor};

lazy_static::lazy_static! {
    pub static ref UNDEFINED: Arc<dyn Basic> = Arc::new(Undefined);
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Undefined;

impl Basic for Undefined {
    fn contains_symbol(&self, _sym: &Symbol) -> bool {
        false
    }

    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_undefined(self)
    }

    fn eq(&self, other: &dyn Basic) -> bool {
        other.as_any().is::<Self>()
    }

    fn cmp(&self, other: &dyn Basic) -> Option<Ordering> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            Some(Ord::cmp(self, other))
        } else {
            None
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}