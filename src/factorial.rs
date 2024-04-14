/// Calculate the factorial of a non-negative integer n.
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
/// use bens_number_theory;
///
/// assert_eq!(bens_number_theory::factorial::factorial(0), 1);
/// assert_eq!(bens_number_theory::factorial::factorial(1), 1);
/// assert_eq!(bens_number_theory::factorial::factorial(5), 120);
/// assert_eq!(bens_number_theory::factorial::factorial(10), 3628800);
/// ```
pub fn factorial(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        _ => (2..=n).product(),
    }
}
