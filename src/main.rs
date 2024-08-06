use bens_number_theory::constants::*;
use bens_number_theory::sequences::*;
use num::BigInt;

fn main() {
    let test: num::rational::Ratio<BigInt> = approx_sqrt(2, 50);
    println!("{:?}", test);
}
