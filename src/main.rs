use bens_number_theory::{self, primes::is_mersenne_prime};

fn main() {
    println!("Hello, world!");
    let mut i = 2;
    while i < 32{
        if is_mersenne_prime((2_u128.pow(i)) - 1){
            println!("{}",i);
        }
        i += 1;
    }
}
