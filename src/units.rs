use std::sync::Arc;

use num::rational::Ratio;



pub struct Units {
    // convert(x) := coef * x + offset
    coef: Ratio<usize>,
    offset: Ratio<usize>,
    
    // exponents for SI base units,  e.g. 1/m => meters: -1,
    meters: i32,
    seconds: i32,
    moles: i32,
    amperes: i32,
    kelvins: i32,
    candelas: i32,
    kilograms: i32,
}
