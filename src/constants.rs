use crate::factorials::factorial;
use num::{rational::Ratio, BigInt, BigRational, FromPrimitive};

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
/// assert!(estimate_pi_ratio(1_i128).to_string().starts_with("158853645"));
/// assert!(estimate_pi_ratio(1_u8).to_string().contains('/'));
/// assert!(estimate_pi_ratio(1).to_string().ends_with("899151951"));
/// ```
pub fn estimate_pi_ratio<T>(n: T) -> BigRational
where
    T: num::FromPrimitive + std::cmp::PartialOrd,
{
    // A
    let a_root_two: BigRational = approx_sqrt(2, 10_usize);
    let a_two: BigRational = BigRational::from_integer(FromPrimitive::from_u64(2).unwrap());
    let a_denom: BigRational = BigRational::from_integer(FromPrimitive::from_u64(9801).unwrap());
    let a: BigRational = (a_two * a_root_two) / a_denom;

    let mut i: u32 = 0;
    let mut sum: BigRational = BigRational::from_integer(FromPrimitive::from_u64(0).unwrap());
    while T::from_u32(i).unwrap() < n {
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
///
/// # Example
///
/// ```
/// use num::{rational::Ratio, BigInt, BigRational, FromPrimitive};
/// fn approx_sqrt(number: u64, iterations: usize) -> BigRational {
///     let start: Ratio<BigInt> = Ratio::from_integer(FromPrimitive::from_u64(number).unwrap());
///     let mut approx = start.clone();
///     
///     for _ in 0..iterations {
///         approx = (&approx + (&start / &approx))
///             / Ratio::from_integer(FromPrimitive::from_u64(2).unwrap());
///     }
///     approx
/// }
///
/// println!("{}", approx_sqrt(100_u64, 10_usize));
/// ```
pub fn approx_sqrt<T>(number: u64, iterations: usize) -> Ratio<T>
where
    T: std::ops::Div<Output = T> + std::clone::Clone + num::FromPrimitive + num::Integer,
{
    let start: Ratio<T> = Ratio::from(T::from_u64(number).unwrap());
    let mut approx = start.clone();

    for _ in 0..iterations {
        approx = (&approx + (&start / &approx)) / Ratio::from(T::from_u8(2).unwrap());
    }
    approx
}

/// Uses the Binet's formula to estimate the golden ratio.
///
/// $$\varphi = 1 + \cfrac{1}{1 + \cfrac{1}{1 + \cfrac{1}{1 + \ddots}}}.$$
///
/// Note: This Function specifically uses the Lucas Sequence.
///
/// Let $S$ be a Fibonacci-like sequence of size $n$.
/// $$\lim_{n\to\infty} (S\[n\] / S\[n-1\]) = \varphi$$
///
/// *Binet's formula states that the ratio for two sequential
/// Fibonacci-like indices will converge to the golden ratio.*
///
/// ## Closed-form Expression of Binet's formula
///
/// The following is adapted from [this](https://doi.org/10.1007/978-3-322-85165-9_6) book and [Wikipedia](https://en.wikipedia.org/wiki/Fibonacci_sequence#Closed-form_expression).
///
/// $$F_n = \frac{\varphi^n - \psi^n}{\varphi - \psi} = \frac{\varphi^n - \psi^n}{\sqrt{5}},$$
///
/// where
///
/// $$\varphi = \frac{1 + \sqrt{5}}{2} \approx 1.6180339887...$$
///
/// is the golden ratio, and $\Psi$ is its conjugate:
///
/// $$\psi = \frac{1 - \sqrt{5}}{2} = 1- \varphi = -\frac{1}{\varphi} \approx -0.6180339887$$
///
/// Since $\psi = -\varphi^{-1}$, this formula can also be written as
///
/// $$F_n = \frac{\varphi^n - (-\varphi)^{-n}}{\sqrt{5}} = \frac{\varphi^n - (-\varphi)^{-n}}{2\varphi-1}.$$
///
/// Also note that both $\varphi$ and $\psi$ are solutions to the equation $x^2 = x + 1$, and thus $x^n = x^{n-1} + x^{n-2}$.  
/// Using this relation, we know that the powers of $\varphi$ and $\psi$ satisfy the Fibonacci recursion.
///
/// $$\varphi^n = \varphi^{n-1} + \varphi^{n-2}, \\\\
/// \psi^n = \psi^{n-1} + \psi^{n-2}.$$
///
/// Therefor, for any values $a$ and $b$, the sequence defined by
///
/// $$U_n = a\varphi^n + b\psi^n$$
///
/// satisfies the same recurrence,
///
/// $$U_n = a\varphi^n + b\psi^n \\\\
/// U_n = a(\varphi^{n-1} + \varphi^{n-2}) + b(\psi^{n-1} + \psi^{n-2}) \\\\
/// U_n = a\varphi^{n-1} + b\psi^{n-1} + a\varphi^{n-2}  + b\psi^{n-2} \\\\
/// U_n = U_{n-1} + U_{n-2}$$
///
/// If $a$ and $b$ are chosen so that $U_0 = 0$ and $U_1 = 1$, then the resulting sequence $U_n$ must be the Fibonacci sequence.  
/// This is the same as requiring $a$ and $b$ satisfy the system of equations:
///
/// $$\begin{cases}
///     a+b = 0 \\\\
///     \varphi a + \psi b = 1
/// \end{cases}$$
///
/// which has solution
///
/// $$a = \frac{1}{\varphi - \psi} = \frac{1}{\sqrt{5}}, b = -a$$
///
/// producing the required formula.
///
/// Taking the starting values of $U_0$ and $U_1$ to be arbitrary constants, the general solution is then described as:
///
/// $$U_n = a\varphi^n + b\psi^n$$
///
/// where
///
/// $$a = \frac{U_1 - U_0 \psi}{\sqrt{5}} \\\\
/// b = \frac{U_0\varphi - U_1}{\sqrt{5}}$$
///
/// # Example
///
/// ```
/// use num::BigInt;
/// use bens_number_theory::constants::golden_ratio;
///
/// println!("{}", golden_ratio(BigInt::from(10)));
/// ```
pub fn golden_ratio<T>(n: T) -> Ratio<T>
where
    T: std::cmp::PartialOrd
        + num::FromPrimitive
        + num::traits::Zero
        + std::clone::Clone
        + num::Integer
        + std::ops::AddAssign,
{
    if n < T::from_i32(2).unwrap() {
        let val: Ratio<T> = Ratio::from(T::zero());
        return val;
        // return BigRational::from_i32(0_i32).unwrap();
    }
    let mut lucas: Vec<T> = crate::sequences::lucas_sequence(n);
    let numerator: Ratio<T> = Ratio::from(lucas.pop().unwrap());
    let demom: Ratio<T> = Ratio::from(lucas.pop().unwrap());
    numerator / demom
}
