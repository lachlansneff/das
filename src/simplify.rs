use std::ops::ControlFlow;

use crate::{Number, expr::Expr, functions::{Plus, Times}, symbol::Symbol, undefined::Undefined, visitor::Visitor};


pub fn simplify(expr: Expr) -> Expr {
    todo!()
}

struct SimplifyVisitor {
    res: Expr,
}

impl Visitor for SimplifyVisitor {
    fn visit_undefined(&mut self, undef: &Undefined) -> ControlFlow<()> {
        // res = ;
        ControlFlow::Continue(())
    }

    fn visit_number(&mut self, n: &Number) -> ControlFlow<()> {
        todo!()
    }

    fn visit_symbol(&mut self, sym: &Symbol) -> ControlFlow<()> {
        todo!()
    }

    fn visit_plus(&mut self, plus: &Plus) -> ControlFlow<()> {
        todo!()
    }

    fn visit_times(&mut self, times: &Times) -> ControlFlow<()> {
        todo!()
    }
}
