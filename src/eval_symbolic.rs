use std::ops::ControlFlow;

use crate::{Number, canonicalize::canonicalize, expr::{Expr, ExprRef}, functions::{Derivative, Plus, Times}, symbol::Symbol, undefined::{Undefined, UNDEFINED}, visitor::Visitor};


pub fn symbolic_eval(expr: Expr) -> Expr {
    let mut visitor = SymbolicEvalVisitor {
        res: UNDEFINED.clone(),
    };

    match expr.rf().visit(&mut visitor) {
        ControlFlow::Continue(_) => canonicalize(visitor.res),
        ControlFlow::Break(_) => UNDEFINED.clone(),
    }
}

struct SymbolicEvalVisitor {
    res: Expr,
}

impl Visitor for SymbolicEvalVisitor {
    fn visit_undefined(&mut self, _undef: ExprRef<Undefined>) -> ControlFlow<()> {
        self.res = UNDEFINED.clone();
        ControlFlow::Break(())
    }

    fn visit_number(&mut self, n: ExprRef<Number>) -> ControlFlow<()> {
        self.res = n.into_expr();
        ControlFlow::Continue(())
    }

    fn visit_symbol(&mut self, sym: ExprRef<Symbol>) -> ControlFlow<()> {
        self.res = sym.into_expr();
        ControlFlow::Continue(())
    }

    fn visit_plus(&mut self, plus: ExprRef<Plus>) -> ControlFlow<()> {
        let mut v = vec![];
        for term in plus.terms() {
            term.rf().visit(self)?;
            v.push(self.res.clone());
        }

        self.res = Expr::new(Plus::new(v));
        ControlFlow::Continue(())
    }

    fn visit_times(&mut self, times: ExprRef<Times>) -> ControlFlow<()> {
        let mut v = vec![];
        for term in times.terms() {
            term.rf().visit(self)?;
            v.push(self.res.clone());
        }

        self.res = Expr::new(Times::new(v));
        ControlFlow::Continue(())
    }

    fn visit_derivative(&mut self, derivative: ExprRef<Derivative>) -> ControlFlow<()> {
        self.res = derivative.eval()?;
        ControlFlow::Continue(())
    }
}