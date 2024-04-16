pub mod factorials;
pub mod primes;

/// Generates a list containing the proper devisors of a given number
///
/// # Arguments
///
/// * `n` - The number to find the devisors for
///
/// # Returns
///
/// Vector containing all proper devisors of `n`
///
/// # Examples
///
/// ```
/// use bens_number_theory::divisors;
/// assert_eq!(divisors(10), vec![1, 2, 5, 10]);
/// assert_eq!(divisors(20), vec![1, 2, 4, 5, 10, 20]);
/// ```
pub fn divisors(n: i32) -> Vec<i32> {
    let mut d: Vec<i32> = vec![];
    let mut i = 1;
    while i < n / 2 {
        if n % i == 0 {
            d.push(i);
            d.push(n / i);
        }
        i = i + 1;
    }
    d.sort();
    d.dedup();
    return d;
}
