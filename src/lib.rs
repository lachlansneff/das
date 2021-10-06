#![feature(unsize, coerce_unsized, dispatch_from_dyn, arbitrary_self_types)]
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
// mod simplify;
mod eval_symbolic;
mod canonicalize;
mod convert;
// mod simplify;
// mod units;
// mod expr;

pub use self::number::{Number, ZERO, ONE, MINUS_ONE};

#[cfg(test)]
mod tests {
    use crate::{convert::{Convert, LaTex}, eval_symbolic::symbolic_eval, functions::derivative, symbol::sym};
    
    #[test]
    fn it_works() {
        let x = sym("x");

        let a = x.clone() * x.clone() * 2 + 3.5;
        let a_prime = derivative(a.clone(), x);

        println!("{}", LaTex::convert_to_string(symbolic_eval(a_prime)));

        println!("{}", LaTex::convert_to_string(a));
    }
}
