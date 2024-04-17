use num::BigUint;

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
    println!("{}", factorial(BigUint::from(100_u32)));
}

// Reformat this to accept any type, but ALWAYS return bigint (custom type)
pub fn factorial(n: BigUint) -> BigUint {
    match n {
        n if n == BigUint::from(1_u32) => BigUint::from(1_u32),
        _ => n.clone() * factorial(n - BigUint::from(1_u32)),
    }
}
