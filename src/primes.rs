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
/// use bens_number_theory::primes::is_prime;
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
/// use bens_number_theory::primes::generate_primes;
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
/// use bens_number_theory::primes::is_prime_list;
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
/// use bens_number_theory::primes::is_prime_lazy;
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
/// A Mersenne prime is defined as $\forall p \in \mathbb{N}$, if $p$ is prime, then $2^p - 1$ is also prime.
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
/// use bens_number_theory::primes::is_mersenne_prime;
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
