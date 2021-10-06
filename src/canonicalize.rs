use std::ops::ControlFlow;

use crate::{Number, basic::Basic, expr::{Expr, ExprRef}, functions::{Derivative, Plus, Times}, symbol::Symbol, undefined::{Undefined, UNDEFINED}, visitor::Visitor, number::{ZERO, ONE}};

pub fn canonicalize(expr: Expr) -> Expr {
    let mut visitor = CanonicalizeVisitor {
        res: UNDEFINED.clone(),
    };

    match expr.rf().visit(&mut visitor) {
        ControlFlow::Continue(_) => visitor.res,
        ControlFlow::Break(_) => UNDEFINED.clone(),
    }
}

struct CanonicalizeVisitor {
    res: Expr,
}

impl Visitor for CanonicalizeVisitor {
    fn visit_undefined(&mut self, _undef: ExprRef<Undefined>) -> ControlFlow<()> {
        ControlFlow::Break(())
    }

    fn visit_number(&mut self, n: ExprRef<Number>) -> ControlFlow<()> {
        if let Number::Rational(r) = &*n {
            self.res = Expr::new(Number::Integer(r.numer().clone()))
        } else {
            self.res = n.into_expr();
        }

        ControlFlow::Continue(())
    }

    fn visit_symbol(&mut self, sym: ExprRef<Symbol>) -> ControlFlow<()> {
        self.res = sym.into_expr();
        ControlFlow::Continue(())
    }

    fn visit_plus(&mut self, plus: ExprRef<Plus>) -> ControlFlow<()> {
        let mut v = vec![];
        let mut coef: Option<Number> = None;
        for term in plus.terms() {
            term.rf().visit(self);
            let term = self.res.clone();
            
            if let Ok(n) = term.downcast_expr::<Number>() {
                if n.is_zero() {
                    continue;
                }

                if let Some(coef) = coef.as_mut() {
                    *coef = &*coef + &*n;
                } else {
                    coef = Some((&*n).clone());
                }
            } else if let Some(plus_term) = term.downcast::<Plus>() {
                v.extend_from_slice(plus_term.terms());
            } else {
                v.push(term.clone());
            }
        }

        if let Some(coef) = coef {
            v.push(Expr::new(coef));
        }

        self.res = if v.len() == 0 {
            ZERO.clone()
        } else if v.len() == 1 {
            v[0].clone()
        } else {
            Expr::new(Plus::new(v))
        };

        ControlFlow::Continue(())
    }

    fn visit_times(&mut self, times: ExprRef<Times>) -> ControlFlow<()> {
        let mut v = vec![];
        let mut coef: Option<Number> = None;
        for term in times.terms() {
            term.rf().visit(self);
            let term = self.res.clone();
            
            if let Some(n) = term.downcast::<Number>() {
                if n.is_one() {
                    continue;
                } else if n.is_zero() {
                    self.res = ZERO.clone();
                    return ControlFlow::Continue(());
                }

                if let Some(coef) = coef.as_mut() {
                    *coef = &*coef * &*n;
                } else {
                    coef = Some((&*n).clone());
                }
            } else if let Some(times_term) = term.downcast::<Times>() {
                v.extend_from_slice(times_term.terms());
            } else {
                v.push(term.clone());
            }
        }

        if let Some(coef) = coef {
            v.push(Expr::new(coef));
        }

        self.res = if v.len() == 0 {
            ONE.clone()
        } else if v.len() == 1 {
            v[0].clone()
        } else {
            Expr::new(Times::new(v))
        };

        ControlFlow::Continue(())
    }

    fn visit_derivative(&mut self, derivative: ExprRef<Derivative>) -> ControlFlow<()> {
        derivative.visit(self)
    }
}