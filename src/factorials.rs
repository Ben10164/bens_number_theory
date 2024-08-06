/// Calculate the factorial of a number `n`.
///
/// # Arguments
///
/// * `n` - The value of `n` in `n!`.
///
/// # Returns
///
/// `n!`
///
/// # Examples
///
/// ```
/// use bens_number_theory::factorials::factorial;
/// use num::BigInt;
/// use std::str::FromStr;
///
/// assert_eq!(factorial(0_i8), 1_i8);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(BigInt::from(15_u32)), BigInt::from_str("1307674368000").unwrap());
/// ```
pub fn factorial<T>(n: T) -> T
where
    T: num::traits::One
        + std::clone::Clone
        + std::cmp::PartialOrd
        + std::ops::AddAssign
        + std::ops::Sub<Output = T>,
{
    match n {
        // n if n <= BigInt::from(1_u32) => BigInt::from(1_u32),
        i if i <= T::one() => T::one(),
        _ => n.clone() * factorial(n - T::one()),
    }
}

/// Generates a list of factorial values up to a given number `n`.
///
/// # Arguments:
///
/// * `n` - the number up to which we want to calculate the factorials. The function then returns a vector of type `Vec<T>` containing the factorials of numbers from 1 to `n`
///
/// # Returns:
///
/// Vector of values containing the factorial of numbers from 1 up to `n` (inclusive).
///
/// # Examples
///
/// ```
/// use bens_number_theory::factorials::factorial_list;
///
/// let factorials: Vec<u128> = factorial_list(10);
/// assert_eq!(factorials, [1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800])
/// ```
pub fn factorial_list<T>(n: T) -> Vec<T>
where
    T: num::traits::One
        + std::cmp::PartialOrd
        + std::marker::Copy
        + std::ops::AddAssign
        + std::ops::Sub<Output = T>,
{
    let mut f: Vec<T> = vec![];
    let mut i: T = T::one();
    while i <= n {
        f.push(factorial(i));
        i += T::one();
    }
    f
}
