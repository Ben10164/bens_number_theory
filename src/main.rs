use bens_number_theory::sequences::*;

fn main() {
    let test = fib_rec_custom(&[100, 2, 5, 1], 3);
    println!("{:?}", test);
}
