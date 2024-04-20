/// This will contain functions to calculate mathematical constants such as pi, e, etc.
use bigdecimal::BigDecimal;
use num_bigint::BigInt;

use crate::factorials::factorial;

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
pub fn calculate_pi(n: u32) -> BigDecimal {
    // A
    let a_root_two: BigDecimal = BigDecimal::from(2).sqrt().unwrap();
    let a_two: BigDecimal = BigDecimal::from(2);
    let a_denom: BigDecimal = BigDecimal::from(9801);
    let a: BigDecimal = (a_two * a_root_two) / a_denom;

    let mut i: u32 = 0;
    let mut sum: BigDecimal = BigDecimal::from(0);
    while i < n {
        // B_n
        let b_top: BigDecimal =
            BigDecimal::from(factorial(BigInt::from(4 * i)) * (1103 + 26390 * i));
        let b_bot: BigDecimal =
            BigDecimal::from(BigInt::from(4_u32).pow(4 * i) * factorial(BigInt::from(i)).pow(4));
        let b_n: BigDecimal = b_top / b_bot;

        // C_n
        let c_n_big_int: BigInt = BigInt::from(99).pow(4 * i) * (factorial(BigInt::from(i))).pow(4);
        let c_n: BigDecimal = BigDecimal::from(c_n_big_int).inverse();

        sum += b_n * c_n;
        i += 1;
    }

    (a * sum).inverse()
}
