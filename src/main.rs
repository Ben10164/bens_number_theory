use bens_number_theory::constants::*;
use bens_number_theory::sequences::*;
use num::BigInt;

fn main() {
    let sequence: Vec<BigInt> = dying_rabbits_sequence(BigInt::from(5));
    println!("{:?}", sequence);

    println!("{:?}", golden_ratio(12_u8));
    let test: num::rational::Ratio<BigInt> = estimate_pi_ratio(1);
    println!("{:?}", test);
    let t: i8 = 1_i8.pow(1);
}
