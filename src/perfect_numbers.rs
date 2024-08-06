/// Uses the Euclid-Euler theorem to calculate even perfect numbers.
///
/// # Arguments
///
/// * `n` - The max value for p in the formula $(2^{p-1})(2^p-1)$
///
/// Returns
///
/// `Vec<u32>` vector containing perfect numbers
///
/// # Examples
///
/// ```
/// use bens_number_theory::perfect_numbers::generate_even_perfect_numbers;
/// assert_eq!(generate_even_perfect_numbers(10), vec![6, 28, 496, 8128]);
/// ```
pub fn generate_even_perfect_numbers<T>(n: T) -> Vec<T>
where
    T: num::traits::Zero
        + num::traits::One
        + num::FromPrimitive
        + num::ToPrimitive
        + std::ops::Sub<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::AddAssign
        + std::cmp::Ord
        + Copy,
{
    // use the euclid-euler theorem
    // lets start by generating prime numbers up until n
    let primes: Vec<T> = super::primes::generate_primes(n);
    let mut nums: Vec<T> = vec![];
    for prime in primes {
        if super::primes::is_prime(2_i32.pow(prime.to_u32().unwrap()) - 1) {
            nums.push(
                T::from_u32(
                    2_u32.pow((prime - T::one()).to_u32().unwrap())
                        * (2_u32.pow(prime.to_u32().unwrap()) - 1),
                )
                .unwrap(),
            );
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
/// use bens_number_theory::perfect_numbers::is_perfect_number;
/// assert_eq!(is_perfect_number(6), true); // 1 + 2 + 3 = 6
/// assert_eq!(is_perfect_number(8), false); // 1 + 2 + 4 != 8
/// ```
pub fn is_perfect_number<T>(n: T) -> bool
where
    T: num::FromPrimitive
        + num::traits::Zero
        + num::traits::One
        + std::cmp::Ord
        + std::marker::Copy
        + std::ops::AddAssign
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>,
{
    let mut divisors: Vec<T> = divisors(n);
    divisors.pop();
    let mut i: usize = 0;
    let mut sum: T = T::zero();
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
/// use bens_number_theory::perfect_numbers::divisors;
/// assert_eq!(divisors(10), vec![1, 2, 5, 10]);
/// assert_eq!(divisors(20), vec![1, 2, 4, 5, 10, 20]);
/// ```
pub fn divisors<T>(n: T) -> Vec<T>
where
    T: num::FromPrimitive
        + num::traits::One
        + num::traits::Zero
        + std::cmp::Ord
        + std::marker::Copy
        + std::ops::AddAssign
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>,
{
    let mut d: Vec<T> = vec![];
    let mut i: T = T::one();
    while i < n / T::from_i32(2).unwrap() {
        if n % i == T::zero() {
            d.push(i);
            d.push(n / i);
        }
        i += T::one();
    }
    d.sort();
    d.dedup();
    d
}
