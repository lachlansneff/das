use std::{mem, ops::ControlFlow};

use crate::{Number, basic::Basic, expr::{Expr, ExprRef}, number::{ZERO, ONE}, symbol::Symbol, undefined::{UNDEFINED, Undefined}, visitor::Visitor};

use super::{Plus, Times};

pub fn derivative(expr: Expr, wrt: Expr<Symbol>) -> Expr<Derivative> {
    Expr::new(Derivative::new(expr, wrt))
}

#[derive(Debug, Eq, PartialOrd, Ord)]
pub struct Derivative {
    wrt: Expr<Symbol>,
    expr: Expr,
}

impl Derivative {
    fn new(expr: Expr, wrt: Expr<Symbol>) -> Self {
        Self { wrt, expr }
    }

    /// Temporary
    pub fn eval(&self) -> ControlFlow<(), Expr> {
        let mut visitor = DerivVisitor {
            wrt: &*self.wrt,
            res: UNDEFINED.clone(),
        };

        match self.expr.rf().visit(&mut visitor) {
            ControlFlow::Continue(_) => ControlFlow::Continue(visitor.res),
            ControlFlow::Break(_) => ControlFlow::Break(()),
        }
    }
    
    pub fn wrt(&self) -> ExprRef<Symbol> {
        self.wrt.rf()
    }

    pub fn expr(&self) -> ExprRef {
        self.expr.rf()
    }
}

impl Basic for Derivative {
    fn visit(self: ExprRef<Self>, visitor: &mut dyn Visitor) -> ControlFlow<()> {
        visitor.visit_derivative(self)
    }

    fn eq(&self, other: &dyn Basic) -> bool {
        if let Some(other) = other.downcast::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn cmp(&self, other: &dyn Basic) -> Option<std::cmp::Ordering> {
        if let Some(other) = other.downcast::<Self>() {
            Some(Ord::cmp(self, other))
        } else {
            None
        }
    }
}

impl ::core::cmp::PartialEq for Derivative {
    #[inline]
    fn eq(&self, other: &Derivative) -> bool {
        match *other {
            Derivative {
                wrt: ref __self_1_0,
                expr: ref __self_1_1,
            } => match *self {
                Derivative {
                    wrt: ref __self_0_0,
                    expr: ref __self_0_1,
                } => (*__self_0_0) == (*__self_1_0) && (__self_0_1) == (__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Derivative) -> bool {
        match *other {
            Derivative {
                wrt: ref __self_1_0,
                expr: ref __self_1_1,
            } => match *self {
                Derivative {
                    wrt: ref __self_0_0,
                    expr: ref __self_0_1,
                } => (*__self_0_0) != (*__self_1_0) || (__self_0_1) != (__self_1_1),
            },
        }
    }
}

pub struct DerivVisitor<'a> {
    wrt: &'a Symbol,
    res: Expr,
}

impl Visitor for DerivVisitor<'_> {
    fn visit_undefined(&mut self, _undef: ExprRef<Undefined>) -> ControlFlow<()> {
        self.res = UNDEFINED.clone();
        ControlFlow::Break(())
    }

    fn visit_number(&mut self, _n: ExprRef<Number>) -> ControlFlow<()> {
        self.res = ZERO.clone();
        ControlFlow::Continue(())
    }

    fn visit_symbol(&mut self, sym: ExprRef<Symbol>) -> ControlFlow<()> {
        if &*sym == self.wrt {
            self.res = ONE.clone();
        } else {
            self.res = ZERO.clone();
        }
        ControlFlow::Continue(())
    }

    fn visit_plus(&mut self, plus: ExprRef<Plus>) -> ControlFlow<()> {
        let mut terms = vec![];
        for term in plus.terms() {
            term.rf().visit(self)?;
            terms.push(self.res.clone());
        }

        self.res = Expr::new(Plus::new(terms));
        ControlFlow::Continue(())
    }

    fn visit_times(&mut self, times: ExprRef<Times>) -> ControlFlow<()> {
        let mut plus_terms: Vec<Expr> = vec![];
        for i in 0..times.terms().len() {
            let mut times_terms = vec![];
            for (j, term) in times.terms().iter().enumerate() {
                if i == j {
                    term.rf().visit(self)?;
                    times_terms.push(self.res.clone());
                } else {
                    times_terms.push(term.clone());
                }
            }
            plus_terms.push(Expr::new(Times::new(times_terms)));
        }

        self.res = Expr::new(Plus::new(plus_terms));
        ControlFlow::Continue(())
    }

    fn visit_derivative(&mut self, derivative: ExprRef<Derivative>) -> ControlFlow<()> {
        let mut visitor = DerivVisitor {
            wrt: &*self.wrt,
            res: UNDEFINED.clone(),
        };

        derivative.expr.rf().visit(&mut visitor)?;
        self.res = visitor.res;
        ControlFlow::Continue(())
    }
}
