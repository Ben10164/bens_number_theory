use bens_number_theory::sequences::*;
use num::BigInt;

fn main() {
    let sequence: Vec<BigInt> = dying_rabbits_sequence(BigInt::from(5));
    println!("{:?}", sequence);
}
