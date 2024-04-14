use bens_number_theory;

fn main() {
    println!("Enter an i32: ");
    bens_number_theory::read!(n as i32);
    if bens_number_theory::primes::is_prime(n) {
        println!("{} is a prime number", n);
    } else {
        println!("{} is not a prime number", n);
    }
}
