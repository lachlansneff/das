use std::{fmt, io, ops::ControlFlow};

use crate::{Number, expr::{Expr, ExprRef}, functions::{Derivative, Plus, Times}, number::Sign, symbol::Symbol, undefined::Undefined, visitor::Visitor};


pub trait Convert {
    fn write_conversion(w: &mut impl fmt::Write, expr: Expr) -> fmt::Result;

    fn convert_to_string(expr: Expr) -> String {
        let mut s = String::new();
        Self::write_conversion(&mut s, expr).unwrap();
        s
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct LaTex {

}

impl Convert for LaTex {
    fn write_conversion(w: &mut impl fmt::Write, expr: Expr) -> fmt::Result {
        let mut visitor = LaTexVisitor {
            w,
            res: Ok(())
        };

        expr.rf().visit(&mut visitor);

        visitor.res
    }
}

struct LaTexVisitor<W> {
    w: W,
    res: fmt::Result,
}

impl<W: fmt::Write> Visitor for LaTexVisitor<W> {
    fn visit_undefined(&mut self, _undef: ExprRef<Undefined>) -> ControlFlow<()> {
        ControlFlow::Break(())
    }

    fn visit_number(&mut self, n: ExprRef<Number>) -> ControlFlow<()> {
        match &*n {
            Number::Integer(i) => {
                if let Err(e) = write!(self.w, "{}", i) {
                    self.res = Err(e);
                    return ControlFlow::Break(());
                }
            },
            Number::Rational(r) => {
                if let Err(e) = write!(self.w, "\\frac{{{}}}{{{}}}", r.numer(), r.denom()) {
                    self.res = Err(e);
                    return ControlFlow::Break(());
                }
            }
            Number::Infinity(sign) => {
                if let Err(e) = match sign {
                    Sign::Plus => write!(self.w, "+ \\infty"),
                    Sign::Minus => write!(self.w, "- \\infty"),
                } {
                    self.res = Err(e);
                    return ControlFlow::Break(());
                }
            }
        }

        ControlFlow::Continue(())
    }

    fn visit_symbol(&mut self, sym: ExprRef<Symbol>) -> ControlFlow<()> {
        if let Err(e) = write!(self.w, "{}", sym.as_str()) {
            self.res = Err(e);
            return ControlFlow::Break(());
        }
        
        ControlFlow::Continue(())
    }

    fn visit_plus(&mut self, plus: ExprRef<Plus>) -> ControlFlow<()> {
        for (i, term) in plus.terms().iter().enumerate() {
            term.rf().visit(self)?;

            if i < plus.terms().len() - 1 {
                // this is not the last item
                if let Err(e) = write!(self.w, " + ") {
                    self.res = Err(e);
                    return ControlFlow::Break(());
                }
            }
        }

        ControlFlow::Continue(())
    }

    fn visit_times(&mut self, times: ExprRef<Times>) -> ControlFlow<()> {
        for (i, term) in times.terms().iter().enumerate() {
            term.rf().visit(self)?;

            if i < times.terms().len() - 1 {
                // this is not the last item
                if let Err(e) = write!(self.w, " * ") {
                    self.res = Err(e);
                    return ControlFlow::Break(());
                }
            }
        }

        ControlFlow::Continue(())
    }

    fn visit_derivative(&mut self, derivative: ExprRef<Derivative>) -> ControlFlow<()> {
        if let Err(e) = write!(self.w, "\\frac{{d}}{{d{}}}(", derivative.wrt().as_str()) {
            self.res = Err(e);
            return ControlFlow::Break(());
        }

        derivative.expr().visit(self)?;

        if let Err(e) = write!(self.w, ")") {
            self.res = Err(e);
            return ControlFlow::Break(());
        }

        ControlFlow::Continue(())
    }
}