use std::{mem, sync::Arc};

use crate::{basic::Basic, number::{ZERO, ONE}, symbol::Symbol, undefined::{UNDEFINED, Undefined}, visitor::Visitor};

use super::{Plus, Times};


pub struct Derivative {
    wrt: Arc<Symbol>,
    expr: Arc<dyn Basic>,
}

impl Derivative {
    pub fn new(expr: Arc<dyn Basic>, wrt: Arc<Symbol>) -> Self {
        Self { wrt, expr }
    }

    /// Temporary
    pub fn eval(&self) -> Arc<dyn Basic> {
        let mut visitor = DerivVisitor {
            wrt: &*self.wrt,
            res: UNDEFINED.clone(),
            invalid: false,
        };

        self.expr.visit(&mut visitor);

        if visitor.invalid {
            UNDEFINED.clone()
        } else {
            visitor.res
        }
    }
}

pub struct DerivVisitor<'a> {
    wrt: &'a Symbol,
    res: Arc<dyn Basic>,
    invalid: bool,
}

impl DerivVisitor<'_> {
    fn take(&mut self) -> Arc<dyn Basic> {
        mem::replace(&mut self.res, UNDEFINED.clone())
    }
}

impl Visitor for DerivVisitor<'_> {
    fn visit_undefined(&mut self, _undef: &Undefined) {
        self.invalid = true;
        self.res = UNDEFINED.clone();
    }

    fn visit_number(&mut self, _n: &crate::Number) {
        self.res = ZERO.clone()
    }

    fn visit_symbol(&mut self, sym: &Symbol) {
        if sym == self.wrt {
            self.res = ONE.clone();
        } else {
            self.res = ZERO.clone();
        }
    }

    fn visit_plus(&mut self, plus: &Plus) {
        let mut terms = vec![];
        for term in plus.terms() {
            term.visit(self);
            terms.push(self.take());
        }

        self.res = Arc::new(Plus::new(terms));
    }

    fn visit_times(&mut self, times: &Times) {
        let mut plus_terms: Vec<Arc<dyn Basic>> = vec![];
        for i in 0..times.terms().len() {
            let mut times_terms = vec![];
            for (j, term) in times.terms().iter().enumerate() {
                if i == j {
                    term.visit(self);
                    times_terms.push(self.take());
                } else {
                    times_terms.push(term.clone());
                }
            }
            plus_terms.push(Arc::new(Times::new(times_terms)));
        }

        self.res = Arc::new(Plus::new(plus_terms));
    }
}
