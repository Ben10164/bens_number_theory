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
/// assert_eq!(is_prime(9_i128), false);
/// assert_eq!(is_prime(11_u8), true);
/// ```
pub fn is_prime<T>(n: T) -> bool
where
    T: num::traits::Zero
        + num::traits::One
        + num::FromPrimitive
        + num::ToPrimitive
        + std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::AddAssign
        + std::cmp::Ord
        + Copy,
{
    // account for early false negative/positives
    if n == T::from_u32(2).unwrap() {
        return true;
    } else if n <= T::one() {
        return false;
    }

    let limit: f32 = (n.to_f32().unwrap()).sqrt();
    let p: Vec<T> = generate_primes((n / T::from_i32(2).unwrap()) + T::one());
    for prime in &p {
        if n % *prime == T::zero() {
            return false;
        }
        if prime.to_f32().unwrap() > limit {
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
/// assert_eq!(generate_primes(10_u8), vec![2_u8, 3_u8, 5_u8, 7_u8]);
/// ```
pub fn generate_primes<T>(limit: T) -> Vec<T>
where
    T: num::traits::Zero
        + num::traits::One
        + num::ToPrimitive
        + num::FromPrimitive
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::AddAssign
        + std::cmp::PartialOrd
        + Clone
        + Copy,
{
    if limit < T::zero() {
        panic!();
    }
    let mut p: Vec<T> = vec![T::from_i32(2).unwrap(), T::from_u32(3).unwrap()];
    let mut n: T = T::from_i32(5).unwrap();
    while n < limit {
        if is_prime_list(n, p.clone()) {
            p.push(n);
        }
        n += T::from_i32(2).unwrap();
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
pub fn is_prime_list<T>(n: T, p: Vec<T>) -> bool
where
    T: num::traits::Zero
        + num::traits::One
        + num::ToPrimitive
        + num::FromPrimitive
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::AddAssign
        + std::cmp::PartialOrd
        + Clone
        + Copy,
{
    let limit: f32 = (n.to_f32().unwrap()).sqrt();
    for prime in &p {
        if n % *prime == T::zero() {
            return false;
        }
        if prime.to_f32().unwrap() > limit {
            return true;
        }
    }
    // due to Bertrand's postulate, this should never be reached
    // (if we are calculating sequentially)
    false
}

/// Checks if a given number is a prime.
///
/// Arguments:
///
/// * `m` - Number to check if it is a prime.
///
/// Returns:
///
/// Boolean value indicating whether the input number `m` is a prime or not.
///
/// # Examples
///
/// ```
/// use bens_number_theory::primes::is_prime_lazy;
/// assert_eq!(is_prime_lazy(2_), true);
/// assert_eq!(is_prime_lazy(3_u128), true);
/// assert_eq!(is_prime_lazy(4_i128), false);
/// assert_eq!(is_prime_lazy(5_i32), true);
/// ```
pub fn is_prime_lazy<T>(n: T) -> bool
where
    T: num::traits::Zero
        + num::traits::One
        + num::FromPrimitive
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::AddAssign
        + std::cmp::PartialOrd
        + Copy,
{
    if n == T::one() {
        return false;
    }
    if n == T::from_i32(2).unwrap() {
        return true;
    }
    if n % T::from_i32(2).unwrap() == T::zero() {
        false
    } else {
        let mut i: T = T::from_i32(3).unwrap();
        while i <= n / T::from_i32(2).unwrap() {
            if n % i == T::zero() {
                return false;
            }
            i += T::from_i32(2).unwrap();
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
/// assert_eq!(is_mersenne_prime((2_i32.pow(2)) - 1), true);
/// assert_eq!(is_mersenne_prime((2_u64.pow(3)) - 1), true);
/// assert_eq!(is_mersenne_prime((2_u128.pow(4)) - 1), false);
/// assert_eq!(is_mersenne_prime((2_i128.pow(5)) - 1), true);
/// ```
pub fn is_mersenne_prime<T>(m: T) -> bool
where
    T: num::traits::Zero
        + num::traits::One
        + num::FromPrimitive
        + num::ToPrimitive
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::AddAssign
        + std::cmp::PartialOrd
        + Copy,
{
    if is_prime_lazy(m) && is_prime_lazy((m + T::one()).to_usize().unwrap().ilog2()) {
        return true;
    }
    false
}
