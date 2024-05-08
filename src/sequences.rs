use num::BigInt;

/// Calculates a vector of numbers representing the Lucas Sequence.
///
/// The Lucas Sequence is defined as:
/// $$L_n :=\begin{cases}
///     2                   & \text{if } n = 0; \\\\
///     1                   & \text{if } n = 1; \\\\
///     L_{n-1} + L_{n-2}   & \text{if } n > 1.
/// \end{cases}$$
///
/// $\text{Lucas numbers can also be expressed as the sum of adjacent Fibonacci numbers:}$
///
/// $L_n = \varphi^n + (-1)^n \varphi^{-n} \text{, where } \varphi = \frac{1 + \sqrt{5}}{2} \text{ is the golden ratio.}$
///
/// # Arguments
///
/// `n` - The size of the list to return
///
/// # Example
///
/// ```
/// use bens_number_theory::sequences::lucas_sequence;
/// use num::BigInt;
///
/// let lucas = lucas_sequence(BigInt::from(5));
/// assert_eq!(lucas, [BigInt::from(2),
///     BigInt::from(1), BigInt::from(3),
///     BigInt::from(4), BigInt::from(7)]
/// );
/// ```
pub fn lucas_sequence(n: BigInt) -> Vec<BigInt> {
    match n {
        _ if n == BigInt::from(0) => vec![],
        _ if n == BigInt::from(1) => vec![BigInt::from(2)],
        _ => generate_lucas_sequence(n),
    }
}

/// Function that generates the Lucas sequence used by `lucas_sequence() -> Vec<t>`.
///
/// The Lucas Sequence is defined as:
/// $$L_n :=\begin{cases}
///     2                   & \text{if } n = 0; \\\\
///     1                   & \text{if } n = 1; \\\\
///     L_{n-1} + L_{n-2}   & \text{if } n > 1.
/// \end{cases}$$
///
/// # Arguments
///
/// * `n` - The size of the Lucas sequence to generate.
///
/// # Returns
///
/// A vector containing the Lucas sequence up to the specified size.
///
/// # Example
///
/// ```
/// use num::BigInt;
/// 
/// fn generate_lucas_sequence(n: BigInt) -> Vec<BigInt> {
///     let mut p: Vec<BigInt> = vec![BigInt::from(2), BigInt::from(1)];
///     let mut i: BigInt = BigInt::from(2);
///     while i < n {
///         let last_two: &[BigInt; 2] = p.last_chunk().unwrap();
///         let new: BigInt = last_two.first().unwrap() + last_two.get(1).unwrap();
///         p.push(new);
///         i += 1;
///     }
///     p
/// }
///
/// let lucas = generate_lucas_sequence(BigInt::from(5));
/// assert_eq!(lucas, [BigInt::from(2),
///     BigInt::from(1), BigInt::from(3),
///     BigInt::from(4), BigInt::from(7)]
/// );
/// ```
fn generate_lucas_sequence(n: BigInt) -> Vec<BigInt> {
    let mut p: Vec<BigInt> = vec![BigInt::from(2), BigInt::from(1)];
    let mut i: BigInt = BigInt::from(2);
    while i < n {
        let last_two: &[BigInt; 2] = p.last_chunk().unwrap();
        let new: BigInt = last_two.first().unwrap() + last_two.get(1).unwrap();
        p.push(new);
        i += 1;
    }
    p
}

/// Generate a sequence of dying rabbits based on a given number `n`.
///
/// This function calculates a sequence of dying rabbits based on the number `n`, where each
/// element in the sequence depends on the previous ones. The sequence starts from the 13th
/// element, as it assumes that `n` is greater than or equal to 12.
///
/// The Dying Rabbit Sequence is defined as:  
/// $$R_n :=\begin{cases}
///     0                               & \text{if } n = 0; \\\\
///     F_n                             & \text{if } 1 \le n \le 12; \\\\
///     R_{n-1} + R_{n-2} - R_{n-13}    & \text{if } n > 1.
/// \end{cases}$$
///
///
/// # Arguments
///
/// * `n` - The number representing the length of the sequence to generate.
///
/// # Returns
///
/// A vector containing the sequence of dying rabbits.
///
/// # Example
///
/// ```
/// use num::BigInt;
/// use bens_number_theory::sequences::dying_rabbits_sequence;
///
/// let sequence = dying_rabbits_sequence(BigInt::from(5));
/// assert_eq!(sequence, [BigInt::from(1),
///     BigInt::from(1), BigInt::from(1),
///     BigInt::from(2), BigInt::from(3)]
/// )
/// ```
pub fn dying_rabbits_sequence(n: BigInt) -> Vec<BigInt> {
    let mut i: BigInt = BigInt::from(0);
    let mut nums: Vec<BigInt> = vec![];
    while i < n {
        // println!("{}", decide_dying_rabbit(&i, &nums));
        nums.push(decide_dying_rabbit(i.clone(), &nums));
        i += 1;
    }
    nums
}

/// Determines which of the 3 options to do for the Dying Rabbits sequence.
///
/// Dying rabbits: a(0) = 1; for 1 <= n <= 12, a(n) = Fibonacci(n); for n >= 13, a(n) = a(n-1) + a(n-2) - a(n-13).
///
/// # Arguments
///
/// * `i` - The index of the current element in the sequence.
/// * `nums` - A slice containing the previous elements of the sequence.
///
/// # Returns
///
/// The value of the element in the Dying Rabbits sequence at index `i`.
///
/// # Example
///
/// ```
/// use num::BigInt;
/// use bens_number_theory::sequences::fibonacci_sequence;
/// 
/// fn dying_rec(nums: &[BigInt]) -> BigInt {
///     let last_13: &[BigInt; 13] = nums.last_chunk().unwrap();
///     let new: BigInt =
///         (last_13.get(12).unwrap() + last_13.get(11).unwrap()) - last_13.first().unwrap();
///     new
/// }
/// 
/// fn decide_dying_rabbit(i: BigInt, nums: &[BigInt]) -> BigInt {
///     match i {
///         _ if i == BigInt::from(0) => BigInt::from(1),
///         _ if i < BigInt::from(13) => fibonacci_sequence(i + BigInt::from(1))
///             .last()
///             .unwrap()
///             .to_owned(),
///         _ => dying_rec(nums),
///     }
/// }
///
/// let nums = [BigInt::from(1), BigInt::from(1)];
/// let result = decide_dying_rabbit(BigInt::from(1), &nums);
/// assert_eq!(result, BigInt::from(1));
/// ```
fn decide_dying_rabbit(i: BigInt, nums: &[BigInt]) -> BigInt {
    match i {
        _ if i == BigInt::from(0) => BigInt::from(1),
        _ if i < BigInt::from(13) => fibonacci_sequence(i + BigInt::from(1))
            .last()
            .unwrap()
            .to_owned(),
        _ => dying_rec(nums),
    }
}

/// Like fib_rec, but does a(n) = a(n-1) + a(n-2) - a(n-13)
/// Like fib_rec, but calculates the Dying Rabbits sequence according to the formula: a(n) = a(n-1) + a(n-2) - a(n-13).
///
/// # Arguments
///
/// * `nums` - A slice containing the previous 13 elements of the sequence.
///
/// # Returns
///
/// The value of the element in the Dying Rabbits sequence calculated according to the formula.
///
/// $a_n = a_{n-1} + a_{n-2} - a_{n-13}$
///
/// # Example
///
/// ```
/// use num::BigInt;
/// 
/// fn dying_rec(nums: &[BigInt]) -> BigInt {
///     let last_13: &[BigInt; 13] = nums.last_chunk().unwrap();
///     let new: BigInt =
///         (last_13.get(12).unwrap() + last_13.get(11).unwrap()) - last_13.first().unwrap();
///     new
/// }
///
/// let nums = [BigInt::from(1), BigInt::from(1), BigInt::from(2), BigInt::from(3),
///             BigInt::from(4), BigInt::from(6), BigInt::from(9), BigInt::from(13),
///             BigInt::from(19), BigInt::from(28), BigInt::from(41), BigInt::from(60),
///             BigInt::from(88), BigInt::from(129)];
/// let result = dying_rec(&nums);
/// assert_eq!(result, BigInt::from(216));
/// ```
fn dying_rec(nums: &[BigInt]) -> BigInt {
    let last_13: &[BigInt; 13] = nums.last_chunk().unwrap();
    let new: BigInt =
        (last_13.get(12).unwrap() + last_13.get(11).unwrap()) - last_13.first().unwrap();
    new
}

/// Calculates a vector of numbers representing the Fibonacci Sequence.
///
/// The Fibonacci Sequence is defined as:  
/// $$F_n :=\begin{cases}
///     0                   & \text{if } n = 0; \\\\
///     1                   & \text{if } n = 1; \\\\
///     F_{n-1} + F_{n-2}   & \text{if } n > 1.
/// \end{cases}$$
///
/// # Arguments
///
/// `n` - The size of the list to return
///
/// # Returns
///
/// A vector containing the Fibonacci Sequence of length `n`
///
/// # Example
///
/// ```
/// use num::BigInt;
/// use bens_number_theory::sequences::fibonacci_sequence;
///
/// let sequence = fibonacci_sequence(BigInt::from(5));
/// assert_eq!(sequence, [BigInt::from(0),
///     BigInt::from(1), BigInt::from(1),
///     BigInt::from(2), BigInt::from(3)]
/// );
/// ```
pub fn fibonacci_sequence(n: BigInt) -> Vec<BigInt> {
    let mut i: BigInt = BigInt::from(2);
    let mut nums: Vec<BigInt> = vec![BigInt::from(0), BigInt::from(1)];

    while i < n {
        let new: BigInt = fib_rec(&nums);
        nums.push(new);
        i += 1;
        // println!("{:?}", nums);
        // println!("{}", i);
    }
    nums
}

/// Calculate the next Fibonacci number recursively based on the last two numbers.
///
/// This function takes a slice of `BigInt` numbers representing the last two Fibonacci numbers
/// and returns the next Fibonacci number in the sequence.
///
/// # Arguments
///
/// * `nums` - A slice containing the last two Fibonacci numbers.
///
/// # Returns
///
/// The next Fibonacci number in the sequence.
///
/// # Examples
///
/// ```
/// use num::BigInt;
/// 
/// fn fib_rec(nums: &[BigInt]) -> BigInt {
///     let last_two: &[BigInt; 2] = nums.last_chunk().unwrap();
///     let new: BigInt = last_two.first().unwrap() + last_two.get(1).unwrap();
///     new
/// }
///
/// let last_two = [BigInt::from(1), BigInt::from(1)];
/// let next_fib = fib_rec(&last_two);
/// assert_eq!(next_fib, BigInt::from(2));
/// ```
fn fib_rec(nums: &[BigInt]) -> BigInt {
    let last_two: &[BigInt; 2] = nums.last_chunk().unwrap();
    let new: BigInt = last_two.first().unwrap() + last_two.get(1).unwrap();
    new
}
