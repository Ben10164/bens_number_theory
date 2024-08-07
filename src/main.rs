use bens_number_theory::constants::*;

fn main() {
    let test = golden_ratio(1000);
    println!("{:?}", dashu::Decimal::from(test).to_string());
}
