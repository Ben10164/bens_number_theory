use bens_number_theory::{self, generate_even_perfect_numbers, is_mersenne_prime};

fn main() {
    println!("Hello, world!");
    let mut i: u32 = 2;
    while i < 20 {
        if is_mersenne_prime((2_u128.pow(i)) - 1) {
            println!("{}", i);
        }
        i += 1;
    }

    println!("{:?}", generate_even_perfect_numbers(10));
}
