use bens_number_theory::constants::*;
use bens_number_theory::ratio_to_str;

fn main() {
    let test: num::rational::Ratio<num::BigInt> = golden_ratio(100);
    println!("{:?}", test);
    println!("{}", ratio_to_str(test));
}
