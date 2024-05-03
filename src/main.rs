use bens_number_theory::constants::{golden_ratio, lucas_sequence};
use bens_number_theory::factorials::*;
use num::BigInt;

fn main() {
    println!("Hello, world!");
    let x = 1000;
    // println!("{:?}", lucas_sequence(BigInt::from(x)));
    println!("{}", golden_ratio(BigInt::from(x)));
    // println!("{}", factorial(BigInt::from(10000_u16)))
}
