use is_prime;

fn main() {
    println!("Enter an i32: ");
    is_prime::read!(limit as i32);
    let p: Vec<i32> = is_prime::generate_primes(limit);
    println!("{:?}",p);
}

