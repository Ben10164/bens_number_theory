/// Functions that mathematically generate mathematical constants
pub mod constants;
/// Functions related to factorial generation
pub mod factorials;
/// Functions related to perfect numbers
pub mod perfect_numbers;
/// Functions related to prime numbers
pub mod primes;
/// Functions that generate mathematical sequences
pub mod sequences;

pub fn ratio_to_str(ratio: num::rational::BigRational) -> String {
    let numer_str = ratio.numer().to_str_radix(10);
    let numer = dashu::integer::IBig::from_str_radix(&numer_str, 10).unwrap();

    let denom_str = ratio.denom().to_str_radix(10);
    let denom = dashu::integer::UBig::from_str_radix(&denom_str, 10).unwrap();

    let frac1 = dashu::Rational::from_parts(numer, denom);
    dashu::Decimal::from(frac1).to_string()
}
