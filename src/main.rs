use bens_number_theory::{self, primes::is_mersenne_prime};

fn main() {
    // println!("Enter an i32: ");
    // bens_number_theory::read!(n as i32);
    // if bens_number_theory::primes::is_prime(n) {
    //     println!("{} is a prime number", n);
    // } else {
    //     println!("{} is not a prime number", n);
    // }

    println!("Hello, world!");
    let mut i = 2;
    while i < 32{
        if is_mersenne_prime((2_u128.pow(i)) - 1){
            println!("{}",i);
        }
        i += 1;
    }
}
