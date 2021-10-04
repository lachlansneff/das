
mod basic;
mod number;
// mod assumptions;
mod symbol;
mod functions;
// mod diff;
mod visitor;
mod undefined;
// mod units;
// mod expr;

pub use self::number::{Number, ZERO, ONE, MINUS_ONE};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
