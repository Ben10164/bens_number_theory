pub mod factorials;
pub mod primes;

/// Uses the Euclid-Euler theorem to calculate even perfect numbers
///
/// # Arguments
///
/// * `n` - The max value for p in the formula (2^{p-1})(2^p-1)
///
/// Returns
///
/// Vec<u32> vector containing perfect numbers
///
/// # Examples
///
/// ```
/// use bens_number_theory::generate_even_perfect_numbers;
/// assert_eq!(generate_even_perfect_numbers(10), vec![6, 28, 496, 8128]);
/// ```
pub fn generate_even_perfect_numbers(n: i32) -> Vec<u32> {
    // use the euclid-euler theorem
    // lets start by generating prime numbers up until n
    let primes: Vec<i32> = primes::generate_primes(n);
    let mut nums: Vec<u32> = vec![];
    for prime in primes {
        if primes::is_prime(2_i32.pow(prime as u32) - 1) {
            nums.push(2_u32.pow((prime - 1) as u32) * (2_u32.pow(prime as u32) - 1));
        }
    }
    return nums;
}

/// Determines wether or not a given number is a "perfect number"
///
/// `n` is a perfect number if the sum of all proper devisors of `n` results in `n`.
///
/// # Arguments
///
/// * `n` - The number to check perfectness of
///
/// Returns
///
/// Boolean representing the perfectness of `n`
///
/// # Examples
///
/// ```
/// use bens_number_theory::is_perfect_number;
/// assert_eq!(is_perfect_number(6), true); // 1 + 2 + 3 = 6
/// assert_eq!(is_perfect_number(8), false); // 1 + 2 + 4 != 8
/// ```
pub fn is_perfect_number(n: i32) -> bool {
    let mut divisors: Vec<i32> = divisors(n);
    divisors.pop();
    let mut i: usize = 0;
    let mut sum: i32 = 0;
    while i < divisors.len() {
        sum = sum + divisors[i];
        i = i + 1;
    }
    if sum == n {
        return true;
    }
    return false;
}

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
    let mut i: i32 = 1;
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
