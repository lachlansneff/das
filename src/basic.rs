use std::{any::Any, cmp::Ordering, fmt::Debug};

use crate::{symbol::Symbol, visitor::Visitor};

pub trait Basic: Send + Sync + Debug + Any {
    fn contains_symbol(&self, sym: &Symbol) -> bool;
    fn visit(&self, visitor: &mut dyn Visitor);
    fn eq(&self, other: &dyn Basic) -> bool;
    /// In implementations of this, return `None` when
    /// the types do not match.
    fn cmp(&self, other: &dyn Basic) -> Option<Ordering>;
    fn as_any(&self) -> &dyn Any;
}

impl PartialEq for dyn Basic {
    fn eq(&self, other: &Self) -> bool {
        <Self as Basic>::eq(self, other)
    }
}

impl Eq for dyn Basic {}

impl PartialOrd for dyn Basic {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        <Self as Basic>::cmp(self, other)
    }
}

impl Ord for dyn Basic {
    fn cmp(&self, other: &Self) -> Ordering {
        <Self as Basic>::cmp(self, other).unwrap_or_else(|| {
            self.as_any().type_id().cmp(&other.as_any().type_id())
        })
    }
}
