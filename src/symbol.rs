use std::{cmp::Ordering, ops::ControlFlow};

use string_cache::DefaultAtom;

use crate::{Number, basic::Basic, expr::Expr, functions::{Plus, Times}, undefined::Undefined, visitor::Visitor};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol {
    name: DefaultAtom,
}

impl Symbol {
    pub fn new(s: &str) -> Expr<Self> {
        Expr::new(Self {
            name: DefaultAtom::from(s),
        })
    }

    pub fn as_str(&self) -> &str {
        self.name.as_ref()
    }
}

impl Basic for Symbol {
    fn visit(&self, visitor: &mut dyn Visitor) -> ControlFlow<()> {
        visitor.visit_symbol(self)
    }

    fn eq(&self, other: &dyn Basic) -> bool {
        if let Some(other) = other.downcast::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn cmp(&self, other: &dyn Basic) -> Option<Ordering> {
        if let Some(other) = other.downcast::<Self>() {
            Some(Ord::cmp(self, other))
        } else {
            None
        }
    }
}

// pub fn symbol(s: &str) -> Expr<Symbol> {
//     Expr::new(Symbol::new(s))
// }

pub fn depends_on(basic: &dyn Basic, sym: &Symbol) -> bool {
    let mut visitor = DependentSymbolVisitor {
        sym,
        contains: false,
    };

    basic.visit(&mut visitor);

    visitor.contains
}

struct DependentSymbolVisitor<'a> {
    sym: &'a Symbol,
    contains: bool,
}

impl Visitor for DependentSymbolVisitor<'_> {
    fn visit_undefined(&mut self, _undef: &Undefined) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }

    fn visit_number(&mut self, _n: &Number) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }

    fn visit_symbol(&mut self, sym: &Symbol) -> ControlFlow<()> {
        if self.sym == sym {
            self.contains = true;
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }

    fn visit_plus(&mut self, plus: &Plus) -> ControlFlow<()> {
        for term in plus.terms() {
            term.visit(self)?;
        }

        ControlFlow::Continue(())
    }

    fn visit_times(&mut self, times: &Times) -> ControlFlow<()> {
        for term in times.terms() {
            term.visit(self)?;
        }

        ControlFlow::Continue(())
    }
}
