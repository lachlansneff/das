use std::{cmp::Ordering, ops::ControlFlow};

use string_cache::DefaultAtom;

use crate::{Number, basic::Basic, expr::{Expr, ExprRef}, functions::{Derivative, Plus, Times}, undefined::Undefined, visitor::Visitor};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol {
    name: DefaultAtom,
}

impl Symbol {
    fn new(s: &str) -> Self {
        Self {
            name: DefaultAtom::from(s),
        }
    }

    pub fn as_str(&self) -> &str {
        self.name.as_ref()
    }
}

impl Basic for Symbol {
    fn visit(self: ExprRef<Self>, visitor: &mut dyn Visitor) -> ControlFlow<()> {
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

pub fn sym(s: &str) -> Expr<Symbol> {
    Expr::new(Symbol::new(s))
}

pub fn depends_on(basic: ExprRef, sym: &Symbol) -> bool {
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
    fn visit_undefined(&mut self, _undef: ExprRef<Undefined>) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }

    fn visit_number(&mut self, _n: ExprRef<Number>) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }

    fn visit_symbol(&mut self, sym: ExprRef<Symbol>) -> ControlFlow<()> {
        if self.sym == &*sym {
            self.contains = true;
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }

    fn visit_plus(&mut self, plus: ExprRef<Plus>) -> ControlFlow<()> {
        for term in plus.terms() {
            term.rf().visit(self)?;
        }

        ControlFlow::Continue(())
    }

    fn visit_times(&mut self, times: ExprRef<Times>) -> ControlFlow<()> {
        for term in times.terms() {
            term.rf().visit(self)?;
        }

        ControlFlow::Continue(())
    }

    fn visit_derivative(&mut self, derivative: ExprRef<Derivative>) -> ControlFlow<()> {
        derivative.wrt().visit(self)?;
        derivative.expr().visit(self)
    }
}
