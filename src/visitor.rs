use std::ops::ControlFlow;

use crate::{Number, functions::{Plus, Times}, symbol::Symbol, undefined::Undefined};


pub trait Visitor {
    fn visit_undefined(&mut self, undef: &Undefined) -> ControlFlow<()>;
    fn visit_number(&mut self, n: &Number) -> ControlFlow<()>;
    fn visit_symbol(&mut self, sym: &Symbol) -> ControlFlow<()>;
    fn visit_plus(&mut self, plus: &Plus) -> ControlFlow<()>;
    fn visit_times(&mut self, times: &Times) -> ControlFlow<()>;
}
