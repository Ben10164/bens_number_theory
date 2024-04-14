use bens_is_prime;

fn main() {
    println!("Enter an i32: ");
    bens_is_prime::read!(n as i32);
    if bens_is_prime::is_prime(n) {
        println!("{} is a prime number", n);
    }else{
        println!("{} is not a prime number", n);
    }
}

