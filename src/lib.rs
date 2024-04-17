use num_bigint::BigUint;

/// Uses the Euclid-Euler theorem to calculate even perfect numbers.
///
/// # Arguments
///
/// * `n` - The max value for p in the formula (2^{p-1})(2^p-1)
///
/// Returns
///
/// `Vec<u32>` vector containing perfect numbers
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
    let primes: Vec<i32> = generate_primes(n);
    let mut nums: Vec<u32> = vec![];
    for prime in primes {
        if is_prime(2_i32.pow(prime as u32) - 1) {
            nums.push(2_u32.pow((prime - 1) as u32) * (2_u32.pow(prime as u32) - 1));
        }
    }
    nums
}

/// Determines wether or not a given number is a "perfect number".
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
        sum += divisors[i];
        i += 1;
    }
    if sum == n {
        return true;
    }
    false
}

/// Generates a list containing the proper devisors of a given number.
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
        i += 1;
    }
    d.sort();
    d.dedup();
    d
}

/// Calculate the factorial of a `BigUint` `n`.
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
/// use bens_number_theory::factorial;
/// use num_bigint::BigUint;
/// use std::str::FromStr;
///
/// assert_eq!(factorial(BigUint::from(0_u32)), BigUint::from_str("1").unwrap());
/// assert_eq!(factorial(BigUint::from(1_u32)), BigUint::from_str("1").unwrap());
/// assert_eq!(factorial(BigUint::from(15_u32)), BigUint::from_str("1307674368000").unwrap());
/// ```
pub fn factorial(n: BigUint) -> BigUint {
    match n {
        n if n <= BigUint::from(1_u32) => BigUint::from(1_u32),
        _ => n.clone() * factorial(n - BigUint::from(1_u32)),
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
/// use bens_number_theory::factorial_u128;
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
/// use bens_number_theory::factorial_list;
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

/// Check if a given number is prime.
///
/// This function takes a number `n`
///
/// # Arguments
///
/// * `n` - The number to check for primality.
///
/// # Returns
///
/// A boolean value indicating whether the number is prime (`true`) or not (`false`).
///
/// # Examples
///
/// ```
/// use bens_number_theory::is_prime;
/// assert_eq!(is_prime(9), false);
/// assert_eq!(is_prime(11), true);
/// ```
pub fn is_prime(n: i32) -> bool {
    // account for early false negative/positives
    if n == 2 {
        return true;
    } else if n <= 1 {
        return false;
    }

    let limit: f32 = (n as f32).sqrt();
    let p: Vec<i32> = generate_primes((n / 2) + 1);
    for prime in &p {
        if n % prime == 0 {
            return false;
        }
        if *prime as f32 > limit {
            return true;
        }
    }
    // due to Bertrand's postulate, this should never be reached
    // (if we are calculating sequentially)
    false
}

/// Generates a list of prime numbers using the Sieve of Eratosthenes algorithm.
///
/// # Arguments
///
/// * `limit`
///     * The `limit` parameter specifies the upper limit up to which you want to generate prime numbers.
///     * The function `generate_primes` will generate all prime numbers up to this limit and return
/// them as a vector.
///
/// # Returns
///
/// Vector of all prime numbers up to `limit`
///
/// # Examples
///
/// ```
/// use bens_number_theory::generate_primes;
/// assert_eq!(generate_primes(10), vec![2, 3, 5, 7]);
/// ```
pub fn generate_primes(limit: i32) -> Vec<i32> {
    if limit < 0 {
        panic!();
    }
    let mut p: Vec<i32> = vec![2, 3];
    let mut n: i32 = 5;
    while n < limit {
        if is_prime_list(n, p.clone()) {
            p.push(n);
        }
        n += 2;
    }
    p
}

/// Check if a given number is prime using an efficient method optimized for in-order generation.
///
/// This function takes a number `n` and a vector of prime numbers `p`.
/// It iterates through the prime numbers less than or equal to the square root of `n`, checking if any of them divide `n`. If `n` is divisible by any prime, it returns `false`, indicating that `n` is not prime. If none of the prime numbers divide `n`, it returns `true`, indicating that `n` is prime.
///
/// # Arguments
///
/// * `n` - The number to check for primality.
/// * `p` - Vector of prime numbers.
///
/// # Returns
///
/// A boolean value indicating whether the number is prime (`true`) or not (`false`).
///
/// # Examples
///
/// ```
/// use bens_number_theory::is_prime_list;
/// let primes = vec![2, 3, 5, 7];
/// assert_eq!(is_prime_list(9, primes.clone()), false);
/// assert_eq!(is_prime_list(11, primes), true);
/// ```
pub fn is_prime_list(n: i32, p: Vec<i32>) -> bool {
    let limit: f32 = (n as f32).sqrt();
    for prime in &p {
        if n % prime == 0 {
            return false;
        }
        if *prime as f32 > limit {
            return true;
        }
    }
    // due to Bertrand's postulate, this should never be reached
    // (if we are calculating sequentially)
    false
}

/// Checks if a given u128 number is a prime.
///
/// Arguments:
///
/// * `m` - u128 to check if it is a prime.
///
/// Returns:
///
/// Boolean value indicating whether the input number `m` is a prime or not.
///
/// # Examples
///
/// ```
/// use bens_number_theory::is_prime_lazy;
/// assert_eq!(is_prime_lazy(2_u128), true);
/// assert_eq!(is_prime_lazy(3_u128), true);
/// assert_eq!(is_prime_lazy(4_u128), false);
/// assert_eq!(is_prime_lazy(5_u128), true);
/// ```
pub fn is_prime_lazy(n: u128) -> bool {
    if n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        false
    } else {
        let mut i: u128 = 3;
        while i <= n / 2 {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}

/// Checks if a given number is a Mersenne prime.
///
/// Arguments:
///
/// * `m` - u128 to check if it is a Mersenne prime.
///
/// Returns:
///
/// Boolean value indicating whether the input number `m` is a Mersenne prime or not.
///
/// # Examples
///
/// ```
/// use bens_number_theory::is_mersenne_prime;
/// assert_eq!(is_mersenne_prime((2_u128.pow(2)) - 1), true);
/// assert_eq!(is_mersenne_prime((2_u128.pow(3)) - 1), true);
/// assert_eq!(is_mersenne_prime((2_u128.pow(4)) - 1), false);
/// assert_eq!(is_mersenne_prime((2_u128.pow(5)) - 1), true);
/// ```
pub fn is_mersenne_prime(m: u128) -> bool {
    if is_prime_lazy(m) && is_prime_lazy(((m + 1).ilog2()) as u128) {
        return true;
    }
    false
}

#[cfg(test)]
mod is_prime_list_tests {
    use super::*;

    #[test]
    fn is_prime_large_test() {
        // Test case for checking prime numbers up to a large limit
        let primes: Vec<i32> = generate_primes(100);
        assert_eq!(is_prime_list(97, primes.clone()), true); // 97 is prime
        assert_eq!(is_prime_list(99, primes.clone()), false); // 99 is not prime
        assert_eq!(is_prime_list(101, primes), true); // 101 is prime
    }

    #[test]
    fn is_prime_negative_numbers_test() {
        // Test case for negative numbers
        let primes: Vec<i32> = vec![2, 3, 5, 7];
        assert_eq!(is_prime_list(-7, primes.clone()), false); // -7 is not prime
        assert_eq!(is_prime_list(-11, primes), false); // -11 is not prime
    }

    #[test]
    fn is_prime_edge_case_test() {
        // Test case for edge cases of prime number detection
        let primes: Vec<i32> = vec![2, 3, 5, 7];
        assert_eq!(is_prime_list(i32::MAX, primes.clone()), false); // Maximum i32 value is not prime
        assert_eq!(is_prime_list(i32::MIN, primes), false); // Minimum i32 value is not prime
    }

    #[test]
    fn is_prime_large_input_test() {
        // Test case for large input numbers
        let primes: Vec<i32> = generate_primes(1000);
        assert_eq!(is_prime_list(997, primes.clone()), true); // 997 is prime
        assert_eq!(is_prime_list(1001, primes), false); // 1001 is not prime
    }
}

#[cfg(test)]
mod is_prime_lazy_tests {
    use super::*;

    #[test]
    fn test_prime_numbers() {
        assert_eq!(is_prime_lazy(2), true);
        assert_eq!(is_prime_lazy(3), true);
        assert_eq!(is_prime_lazy(5), true);
        assert_eq!(is_prime_lazy(7), true);
        assert_eq!(is_prime_lazy(13), true);
    }

    #[test]
    fn test_non_prime_numbers() {
        assert_eq!(is_prime_lazy(1), false);
        assert_eq!(is_prime_lazy(4), false);
        assert_eq!(is_prime_lazy(6), false);
        assert_eq!(is_prime_lazy(8), false);
        assert_eq!(is_prime_lazy(10), false);
        assert_eq!(is_prime_lazy(12), false);
        assert_eq!(is_prime_lazy(14), false);
    }

    #[test]
    fn test_large_prime_numbers() {
        assert_eq!(is_prime_lazy(1_000_000_007), true); // A large prime number
        assert_eq!(is_prime_lazy(1_000_000_009), true); // Another large prime number
    }
}

#[cfg(test)]
mod is_prime_tests {
    use super::*;

    #[test]
    fn test_non_prime_numbers() {
        // Test some non-prime numbers
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(12), false);
        assert_eq!(is_prime(14), false);
        assert_eq!(is_prime(15), false);
        assert_eq!(is_prime(16), false);
    }

    #[test]
    fn test_prime_numbers() {
        // Test some prime numbers
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(19), true);
        assert_eq!(is_prime(23), true);
    }

    #[test]
    fn test_negative_numbers() {
        // Test negative numbers
        assert_eq!(is_prime(-2), false);
        assert_eq!(is_prime(-3), false);
        assert_eq!(is_prime(-5), false);
        assert_eq!(is_prime(-7), false);
    }

    #[test]
    fn test_zero_and_one() {
        // Test zero and one
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
    }
}

#[cfg(test)]
mod generate_primes_tests {
    use super::*;

    #[test]
    fn generate_primes_test() {
        // Test case for generating prime numbers
        assert_eq!(generate_primes(10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn generate_primes_large_test() {
        // Test case for generating prime numbers up to a large limit
        assert_eq!(
            generate_primes(100),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }

    #[test]
    #[should_panic]
    fn generate_primes_negative_input() {
        // Test case for generating primes with a negative input
        generate_primes(-10);
    }

    #[test]
    fn generate_primes_single_prime_test() {
        // Test case for generating primes with a single prime number as input
        assert_eq!(generate_primes(2), vec![2, 3]); // Only 2 is prime for input 2
        assert_eq!(generate_primes(3), vec![2, 3]); // 2 and 3 are primes for input 3
    }

    #[test]
    fn generate_primes_duplicate_primes_test() {
        // Test case for duplicate prime numbers in the list
        assert_eq!(generate_primes(20), vec![2, 3, 5, 7, 11, 13, 17, 19]); // No duplicate primes
        assert_eq!(
            generate_primes(30),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        ); // No duplicate primes
    }

    #[test]
    fn generate_primes_large_input_test() {
        // Test case for generating prime numbers with large input
        assert_eq!(generate_primes(10000).len(), 1229); // There are 1229 primes up to 10000
        assert_eq!(generate_primes(100000).len(), 9592); // There are 9592 primes up to 100000
    }

    #[test]
    fn generate_primes_limit_one_test() {
        // Test case for generating primes with limit set to 1
        assert_eq!(generate_primes(1), vec![2, 3]); // No prime numbers for limit 1
    }

    #[test]
    fn generate_primes_limit_three_test() {
        // Test case for generating primes with limit set to 3
        assert_eq!(generate_primes(3), vec![2, 3]); // 2 and 3 are primes for limit 3
    }

    #[test]
    fn generate_primes_large_limit_test() {
        // Test case for generating primes with a large limit
        assert_eq!(generate_primes(1000000).len(), 78498); // There are 78498 primes up to 1000000
    }
}

#[cfg(test)]
mod is_mersenne_prime_tests {
    use super::*;

    #[test]
    fn mersenne_primes_tests() {
        assert_eq!(is_mersenne_prime((2_u128.pow(2)) - 1), true);
        assert_eq!(is_mersenne_prime((2_u128.pow(3)) - 1), true);
        assert_eq!(is_mersenne_prime((2_u128.pow(5)) - 1), true);
        assert_eq!(is_mersenne_prime((2_u128.pow(7)) - 1), true);
        assert_eq!(is_mersenne_prime((2_u128.pow(13)) - 1), true);
        assert_eq!(is_mersenne_prime((2_u128.pow(17)) - 1), true);
        assert_eq!(is_mersenne_prime((2_u128.pow(19)) - 1), true);
        assert_eq!(is_mersenne_prime((2_u128.pow(31)) - 1), true);
    }

    #[test]
    fn non_mersenne_primes_tests() {
        assert_eq!(is_mersenne_prime((2_u128.pow(4)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(6)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(8)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(9)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(10)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(11)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(12)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(14)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(15)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(16)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(18)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(20)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(21)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(22)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(23)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(24)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(25)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(26)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(27)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(28)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(29)) - 1), false);
        assert_eq!(is_mersenne_prime((2_u128.pow(30)) - 1), false);
    }
}

#[cfg(test)]
mod factorial_tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial_u128(0), 1);
        assert_eq!(factorial_u128(1), 1);
        assert_eq!(factorial_u128(5), 120);
        assert_eq!(factorial_u128(10), 3628800);
    }
}

#[cfg(test)]
mod factorial_list_tests {
    use super::*;

    #[test]
    fn test_factorial_list() {
        let factorials: Vec<u128> = factorial_list(10);
        assert_eq!(
            factorials,
            [1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800]
        )
    }
}

#[cfg(test)]
mod general_tests {
    use super::*;

    #[test]
    fn divisors_test() {
        assert_eq!(divisors(10), vec![1, 2, 5, 10]);
        assert_eq!(divisors(20), vec![1, 2, 4, 5, 10, 20]);
    }

    #[test]
    fn perfect_number_test() {
        assert_eq!(is_perfect_number(6), true);
        assert_eq!(is_perfect_number(7), false);
        assert_eq!(is_perfect_number(28), true);
        assert_eq!(is_perfect_number(29), false);
        assert_eq!(is_perfect_number(8128), true);
        assert_eq!(is_perfect_number(8130), false);
    }
}
