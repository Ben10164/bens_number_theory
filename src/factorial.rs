/// Calculate the factorial of a non-negative integer `n`.
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
/// use bens_number_theory::factorial::factorial;
///
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(5), 120);
/// assert_eq!(factorial(10), 3628800);
/// ```
pub fn factorial(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        _ => (2..=n).product(),
    }
}

/// Generates a list of factorial values up to a given number `n`.
///
/// Arguments:
///
/// * `n`: `u128`, the number up to which we want to calculate the factorials.
/// The function then returns a vector of type `Vec<u128>` containing the factorials of
/// numbers from 1 to `n`
///
/// Returns:
///
/// Vector of `u128` values containing the factorial of numbers
/// from 1 up to `n` (inclusive).
///
/// Examples
///
/// ```
/// use bens_number_theory::factorial::factorial_list;
///
/// let factorials: Vec<u128> = factorial_list(10);
/// assert_eq!(factorials, [1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800])
/// ```
pub fn factorial_list(n: u128) -> Vec<u128> {
    let mut f: Vec<u128> = vec![];
    let mut i: u128 = 1;
    while i <= n {
        f.push(factorial(i));
        i += 1;
    }
    return f;
}
