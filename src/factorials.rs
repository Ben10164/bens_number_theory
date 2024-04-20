use bigdecimal::num_bigint::BigInt;

/// Calculate the factorial of a `BigInt` `n`.
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
/// use bigdecimal::num_bigint::BigInt;
/// use std::str::FromStr;
///
/// assert_eq!(factorial(BigInt::from(0_u32)), BigInt::from_str("1").unwrap());
/// assert_eq!(factorial(BigInt::from(1_u32)), BigInt::from_str("1").unwrap());
/// assert_eq!(factorial(BigInt::from(15_u32)), BigInt::from_str("1307674368000").unwrap());
/// ```
pub fn factorial(n: BigInt) -> BigInt {
    match n {
        n if n <= BigInt::from(1_u32) => BigInt::from(1_u32),
        _ => n.clone() * factorial(n - BigInt::from(1_u32)),
    }
}

/// Calculate the factorial of a `u128` `n`.
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
/// use bens_number_theory::factorials::factorial_u128;
///
/// assert_eq!(factorial_u128(0), 1);
/// assert_eq!(factorial_u128(1), 1);
/// assert_eq!(factorial_u128(5), 120);
/// assert_eq!(factorial_u128(10), 3628800);
/// ```
pub fn factorial_u128(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        _ => (2..=n).product(),
    }
}

/// Generates a list of factorial values up to a given number `n`.
///
/// # Arguments:
///
/// * `n` - `u128`, the number up to which we want to calculate the factorials. The function then returns a vector of type `Vec<u128>` containing the factorials of numbers from 1 to `n`
///
/// # Returns:
///
/// Vector of `u128` values containing the factorial of numbers from 1 up to `n` (inclusive).
///
/// # Examples
///
/// ```
/// use bens_number_theory::factorials::factorial_list;
///
/// let factorials: Vec<u128> = factorial_list(10);
/// assert_eq!(factorials, [1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800])
/// ```
pub fn factorial_list(n: u128) -> Vec<u128> {
    let mut f: Vec<u128> = vec![];
    let mut i: u128 = 1;
    while i <= n {
        f.push(factorial_u128(i));
        i += 1;
    }
    f
}
