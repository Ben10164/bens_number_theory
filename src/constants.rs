// use crate::factorials::factorial;
// use dashu::integer::{IBig, UBig};
// use num::ToPrimitive;

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
pub fn estimate_pi_ratio<T>(n: T) -> dashu::rational::RBig
where
    T: num::FromPrimitive + std::cmp::PartialOrd,
{
    // A
    let a_root_two: dashu::rational::RBig = approx_sqrt(2, 10_usize);
    let a_two: dashu::rational::RBig = dashu::rational::RBig::from(2);
    let a_denom: dashu::rational::RBig = dashu::rational::RBig::from(9801);
    let a: dashu::rational::RBig = (a_two * a_root_two) / a_denom;

    let mut i: usize = 0;
    let mut sum: dashu::rational::RBig = dashu::rational::RBig::from(0);
    while T::from_usize(i).unwrap() < n {
        // B_n
        let b_top: dashu::integer::IBig = dashu::integer::IBig::from_str_radix(
            &(super::factorials::factorial(num::BigInt::from(4 * i)) * (1103 + 26390 * i))
                .to_str_radix(10),
            10,
        )
        .unwrap();
        let b_bot: dashu::integer::UBig = dashu::integer::UBig::from_str_radix(
            &(num::BigInt::from(4_u32).pow(4 * num::ToPrimitive::to_u32(&i).unwrap())
                * super::factorials::factorial(num::BigInt::from(i)).pow(4))
            .to_str_radix(10),
            10,
        )
        .unwrap();
        let b_n: dashu::rational::RBig = dashu::rational::RBig::from_parts(b_top, b_bot);

        // C_n
        let c_n_og: dashu::rational::RBig = dashu::rational::RBig::from(99).pow(4 * i)
            * (dashu::rational::RBig::from_str_radix(
                &(super::factorials::factorial(num::BigInt::from(i))
                    .pow(4)
                    .to_str_radix(10)),
                10,
            )
            .unwrap());
        let c_n = dashu::rational::RBig::from_parts(
            c_n_og.denominator().as_ibig().clone(),
            c_n_og.numerator().sqr().nth_root(2_usize),
        );
        // let c_n: BigRational = BigRational::from(c_n_big_int).recip();

        sum += b_n * c_n;
        i += 1;
    }
    inverse(a * sum)
}

/// PLEASE MAKE THIS BETTER
fn inverse(frac: dashu::rational::RBig) -> dashu::rational::RBig {
    dashu::rational::RBig::from_parts(
        frac.denominator().as_ibig().clone(),
        frac.numerator().sqr().nth_root(2_usize), // right here
    )
}

/// Function heavily inspired from the num-crate documentation function of the same name
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
fn approx_sqrt<T>(number: T, iterations: usize) -> dashu::rational::RBig
where
    dashu::rational::RBig: From<T>,
{
    let start: dashu::rational::RBig = dashu::rational::RBig::from(number);
    let mut approx = start.clone();

    for _ in 0..iterations {
        approx = (&approx + (&start / &approx))
            / dashu::rational::RBig::simplest_from_f32(2_f32).unwrap();
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
pub fn golden_ratio<T>(n: T) -> dashu::rational::RBig
where
    T: num::FromPrimitive
        + num::traits::One
        + num::traits::Zero
        + std::clone::Clone
        + std::cmp::PartialOrd
        + std::ops::AddAssign,
    num::BigInt: From<T>,
{
    if n < T::from_i32(2).unwrap() {
        return dashu::rational::RBig::from_parts(
            dashu::integer::IBig::ZERO,
            dashu::integer::UBig::ONE,
        );
    }
    let mut lucas = crate::sequences::lucas_sequence(n);
    let numer = lucas.pop().unwrap();
    let denom = lucas.pop().unwrap();
    dashu::rational::RBig::from_parts(
        dashu::integer::IBig::from_str_radix(&numer.to_str_radix(10), 10).unwrap(),
        dashu::integer::UBig::from_str_radix(&denom.to_str_radix(10), 10).unwrap(),
    )
}
