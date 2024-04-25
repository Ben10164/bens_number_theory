use bens_number_theory::constants::estimate_pi_ratio;
use num::{BigRational, FromPrimitive};
use std::f64::consts::PI;

fn main() {
    println!("Hello, world!");
    // let mut i: u32 = 2;
    // while i < 20 {
    //     if is_mersenne_prime((2_u128.pow(i)) - 1) {
    //         println!("{}", i);
    //     }
    //     i += 1;
    // }

    // println!("{:?}", generate_even_perfect_numbers(10));

    // let mut i: BigUint = BigUint::new(vec![1]);
    // while i <= BigUint::new(vec![0, 2]) {
    //     i = i + BigUint::new(vec![100000]);
    // }
    // println!("{}", i);
    // println!("{}", factorial(BigUint::from_str("10").unwrap()));
    // println!("{}", factorial(BigUint::from(22222_u32)));
    println!("{}", estimate_pi_ratio(1));
    println!("{}", BigRational::from_f64(PI).unwrap());
    println!("{:?}", estimate_pi_ratio(1).to_string().starts_with("158853645"));
    println!("{:?}", estimate_pi_ratio(1).to_string().contains('/'));
    println!("{:?}", estimate_pi_ratio(1).to_string().ends_with("899151951"));

assert!(estimate_pi_ratio(1).to_string().starts_with("158853645"));
assert!(estimate_pi_ratio(1).to_string().contains('/'));
assert!(estimate_pi_ratio(1).to_string().ends_with("899151951"));

    // println!("{}", PI);
}
