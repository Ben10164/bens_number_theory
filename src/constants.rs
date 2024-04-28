use crate::factorials::factorial;
use num::{
    bigint::{self, ToBigInt},
    cast::AsPrimitive,
    rational::Ratio,
    BigInt, BigRational, FromPrimitive,
};

/// Calculate a ratio representing the value of $\pi$ using the *Ramanujan–Sato series*
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
/// A BigRational representing $\pi$.
///
/// # Examples
///
/// ```
/// use bens_number_theory::constants::estimate_pi_ratio;
///
/// assert!(estimate_pi_ratio(1).to_string().starts_with("158853645"));
/// assert!(estimate_pi_ratio(1).to_string().contains('/'));
/// assert!(estimate_pi_ratio(1).to_string().ends_with("899151951"));
/// ```
pub fn estimate_pi_ratio(n: u32) -> BigRational {
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

/// Function taken from the num-crate documentation
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

/// Uses the theorem that the ratio for two sequential Fibinacci-like indecies
/// will converge to the golden ratio
/// This specfically uses the Lucas Sequence
pub fn golden_ratio(n: BigInt) -> Ratio<BigInt> {
    if n < BigInt::from(2) {
        return BigRational::from_i32(0_i32).unwrap();
    }
    let mut lucas: Vec<BigInt> = lucas_sequence(n);
    let numerator: Ratio<BigInt> = BigRational::from(lucas.pop().unwrap());
    let demom: Ratio<BigInt> = BigRational::from(lucas.pop().unwrap());
    return numerator / demom;
}

/// Calculates a vector of numbers representing the Lucas Sequence.
///
/// The Lucas Sequence is defined as:
/// $$L_n :=\begin{cases}
///     2                   & \text{if } n = 0; \\\\
///     1                   & \text{if } n = 1; \\\\
///     L_{n-1} + L_{n-2}   & \text{if } n > 1.
/// \end{cases}$$
///
/// $\text{Lucas numbers can also be expressed as the sum of adjacent Fibonacci numbers:}$
///
/// $L_n = \varphi^n + (-1)^n \varphi^{-n} \text{, where } \varphi = \frac{1 + \sqrt{5}}{2} \text{ is the golden ratio.}$
///
/// # Arguments
///
/// `n` - The size of the list to return
pub fn lucas_sequence(n: BigInt) -> Vec<BigInt> {
    match n {
        _ if n == BigInt::from(0) => vec![],
        _ if n == BigInt::from(1) => vec![BigInt::from(2)],
        _ => generate_lucas_sequence(n),
    }
}

/// Private function that does the logic to generate the lucas sequence used by `lucas_sequence() -> Vec<t>`
fn generate_lucas_sequence(n: BigInt) -> Vec<BigInt> {
    let mut p: Vec<BigInt> = vec![BigInt::from(2), BigInt::from(1)];
    let mut i: BigInt = BigInt::from(2);
    while i < n {
        let last_two: &[BigInt; 2] = p.last_chunk().unwrap();
        let new: BigInt = last_two.get(0).unwrap() + last_two.get(1).unwrap();
        p.push(new);
        i += 1;
    }
    p
}
