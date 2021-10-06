use std::ops::ControlFlow;

use crate::{Number, expr::{Expr, ExprRef}, functions::{Derivative, Plus, Times}, symbol::Symbol, undefined::Undefined};


pub trait Visitor {
    fn visit_undefined(&mut self, undef: ExprRef<Undefined>) -> ControlFlow<()>;
    fn visit_number(&mut self, n: ExprRef<Number>) -> ControlFlow<()>;
    fn visit_symbol(&mut self, sym: ExprRef<Symbol>) -> ControlFlow<()>;
    fn visit_plus(&mut self, plus: ExprRef<Plus>) -> ControlFlow<()>;
    fn visit_times(&mut self, times: ExprRef<Times>) -> ControlFlow<()>;

    fn visit_derivative(&mut self, derivative: ExprRef<Derivative>) -> ControlFlow<()>;
}
