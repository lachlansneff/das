use std::{mem, ops::ControlFlow};

use crate::{expr::Expr, number::{ZERO, ONE}, symbol::Symbol, undefined::{UNDEFINED, Undefined}, visitor::Visitor};

use super::{Plus, Times};


pub struct Derivative {
    wrt: Expr<Symbol>,
    expr: Expr,
}

impl Derivative {
    pub fn new(expr: Expr, wrt: Expr<Symbol>) -> Self {
        Self { wrt, expr }
    }

    /// Temporary
    pub fn eval(&self) -> Expr {
        let mut visitor = DerivVisitor {
            wrt: &*self.wrt,
            res: UNDEFINED.clone(),
        };

        let control_flow = self.expr.visit(&mut visitor);

        if let ControlFlow::Break(_) = control_flow {
            UNDEFINED.clone()
        } else {
            visitor.res
        }
    }
}

pub struct DerivVisitor<'a> {
    wrt: &'a Symbol,
    res: Expr,
}

impl DerivVisitor<'_> {
    fn take(&mut self) -> Expr {
        mem::replace(&mut self.res, UNDEFINED.clone())
    }
}

impl Visitor for DerivVisitor<'_> {
    fn visit_undefined(&mut self, _undef: &Undefined) -> ControlFlow<()> {
        self.res = UNDEFINED.clone();
        ControlFlow::Break(())
    }

    fn visit_number(&mut self, _n: &crate::Number) -> ControlFlow<()> {
        self.res = ZERO.clone();
        ControlFlow::Continue(())
    }

    fn visit_symbol(&mut self, sym: &Symbol) -> ControlFlow<()> {
        if sym == self.wrt {
            self.res = ONE.clone();
        } else {
            self.res = ZERO.clone();
        }
        ControlFlow::Continue(())
    }

    fn visit_plus(&mut self, plus: &Plus) -> ControlFlow<()> {
        let mut terms = vec![];
        for term in plus.terms() {
            term.visit(self)?;
            terms.push(self.take());
        }

        self.res = Expr::new(Plus::new(terms));
        ControlFlow::Continue(())
    }

    fn visit_times(&mut self, times: &Times) -> ControlFlow<()> {
        let mut plus_terms: Vec<Expr> = vec![];
        for i in 0..times.terms().len() {
            let mut times_terms = vec![];
            for (j, term) in times.terms().iter().enumerate() {
                if i == j {
                    term.visit(self)?;
                    times_terms.push(self.take());
                } else {
                    times_terms.push(term.clone());
                }
            }
            plus_terms.push(Expr::new(Times::new(times_terms)));
        }

        self.res = Expr::new(Plus::new(plus_terms));
        ControlFlow::Continue(())
    }
}
