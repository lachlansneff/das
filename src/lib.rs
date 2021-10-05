#![feature(unsize, coerce_unsized, dispatch_from_dyn)]
#![deny(unsafe_op_in_unsafe_fn)]

mod basic;
mod number;
// mod assumptions;
mod symbol;
mod functions;
// mod diff;
mod visitor;
mod undefined;
mod expr;
// mod units;
// mod expr;

pub use self::number::{Number, ZERO, ONE, MINUS_ONE};

#[cfg(test)]
mod tests {
    use crate::{functions::Derivative, symbol::Symbol};

    use super::*;

    #[test]
    fn it_works() {
        let two = ONE.clone() + ONE.clone();
        println!("two: {:?}", two);

        let x = Symbol::new("x");
        let two_plus_x = two + x.clone();
        println!("two_plus_x: {:?}", two_plus_x);

        let d_two_plus_x = Derivative::new(two_plus_x, x).eval();
        println!("d_two_plus_x: {:?}", d_two_plus_x);
    }
}
