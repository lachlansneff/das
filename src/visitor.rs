use crate::{Number, functions::{Plus, Times}, symbol::Symbol, undefined::Undefined};


pub trait Visitor {
    fn visit_undefined(&mut self, undef: &Undefined);
    fn visit_number(&mut self, n: &Number);
    fn visit_symbol(&mut self, sym: &Symbol);
    fn visit_plus(&mut self, plus: &Plus);
    fn visit_times(&mut self, times: &Times);
}
