use crate::factorials::factorial;
use num::{rational::Ratio, BigInt, BigRational, FromPrimitive};

/// Calculate the value of $\pi$ using the *Ramanujan–Sato series*
///
/// $$\frac{1}{\pi} = \frac{2\sqrt{2}}{9801}\sum_{n = 0}^{\infty}\frac{(4n)!(1103 + 26390n)}{4^{4n}(n!)^{4}99^{4n}}$$
/// $$\frac{1}{\pi} =  A\sum_{n = 0}^{\infty}B_{n}C_{n}$$
/// $$\text{where }
/// A = \frac{2\sqrt{2}}{9801},
/// B_{n} = \frac{(4n)!(1103 + 26390n)}{4^{4n}(n!)^{4}},
/// C_{n} = \frac{1}{99^{4n}}$$
///
/// # Arguments
///
/// * `n` - The number of iterations to perform. Higher values give better precision.
///
/// # Returns
///
/// A BigDecimal representing $\pi$.
///
/// # Examples
///
/// ```
/// use bens_number_theory::constants::calculate_pi;
/// use bigdecimal::{BigDecimal, FromPrimitive};
/// use std::f32::consts::PI;
///
/// assert_eq!(calculate_pi(10).round(7), BigDecimal::from_f32(PI).unwrap().round(7))
/// ```
pub fn calculate_pi(n: u32) -> BigRational {
    // A
    let a_root_two: BigRational = approx_sqrt(2, 10_usize);
    let a_two: BigRational = BigRational::from_integer(FromPrimitive::from_u64(2).unwrap());
    let a_denom: BigRational = BigRational::from_integer(FromPrimitive::from_u64(9801).unwrap());
    let a: BigRational = (a_two * a_root_two) / a_denom;

    let mut i: u32 = 0;
    let mut sum: BigRational = BigRational::from_integer(FromPrimitive::from_u64(0).unwrap());
    while i < n {
        // B_n
        let b_top: BigRational =
            BigRational::from(factorial(BigInt::from(4 * i)) * (1103 + 26390 * i));
        let b_bot: BigRational =
            BigRational::from(BigInt::from(4_u32).pow(4 * i) * factorial(BigInt::from(i)).pow(4));
        let b_n: BigRational = b_top / b_bot;

        // C_n
        let c_n_big_int: BigInt = BigInt::from(99).pow(4 * i) * (factorial(BigInt::from(i))).pow(4);
        let c_n: BigRational = BigRational::from(c_n_big_int).recip();

        sum += b_n * c_n;
        i += 1;
    }

    (a * sum).recip()
}

/// Function taken from the num crate documentation
/// Uses Newton’s method to approximate a square root to arbitrary precision
fn approx_sqrt(number: u64, iterations: usize) -> BigRational {
    let start: Ratio<BigInt> = Ratio::from_integer(FromPrimitive::from_u64(number).unwrap());
    let mut approx = start.clone();

    for _ in 0..iterations {
        approx = (&approx + (&start / &approx))
            / Ratio::from_integer(FromPrimitive::from_u64(2).unwrap());
    }

    approx
}
