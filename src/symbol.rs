use std::{any::Any, cmp::Ordering};

use string_cache::DefaultAtom;

use crate::{basic::Basic, visitor::Visitor};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol {
    name: DefaultAtom,
}

impl Symbol {
    pub fn new(s: &str) -> Self {
        Self {
            name: DefaultAtom::from(s),
        }
    }

    pub fn as_str(&self) -> &str {
        self.name.as_ref()
    }
}

impl Basic for Symbol {
    fn contains_symbol(&self, sym: &Symbol) -> bool {
        sym == self
    }

    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.visit_symbol(self);
    }

    fn eq(&self, other: &dyn Basic) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn cmp(&self, other: &dyn Basic) -> Option<Ordering> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            Some(Ord::cmp(self, other))
        } else {
            None
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
