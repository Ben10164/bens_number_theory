#[cfg(test)]
mod tests {
    use bens_number_theory::primes::{generate_primes, is_prime, is_prime_list};

    #[test]
    fn is_prime_test() {
        // 9 is not prime
        assert_eq!(is_prime(9), false);

        // 11 is is_prime_list
        assert_eq!(is_prime(11), true);
    }

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
    fn is_prime_large_test() {
        // Test case for checking prime numbers up to a large limit
        let primes = generate_primes(100);
        assert_eq!(is_prime_list(97, primes.clone()), true); // 97 is prime
        assert_eq!(is_prime_list(99, primes.clone()), false); // 99 is not prime
        assert_eq!(is_prime_list(101, primes), true); // 101 is prime
    }

    #[test]
    #[should_panic]
    fn generate_primes_negative_input() {
        // Test case for generating primes with a negative input
        generate_primes(-10);
    }

    #[test]
    fn is_prime_negative_numbers_test() {
        // Test case for negative numbers
        let primes = vec![2, 3, 5, 7];
        assert_eq!(is_prime_list(-7, primes.clone()), false); // -7 is not prime
        assert_eq!(is_prime_list(-11, primes), false); // -11 is not prime
    }

    #[test]
    fn generate_primes_single_prime_test() {
        // Test case for generating primes with a single prime number as input
        assert_eq!(generate_primes(2), vec![2, 3]); // Only 2 is prime for input 2
        assert_eq!(generate_primes(3), vec![2, 3]); // 2 and 3 are primes for input 3
    }

    #[test]
    fn is_prime_edge_case_test() {
        // Test case for edge cases of prime number detection
        let primes = vec![2, 3, 5, 7];
        assert_eq!(is_prime_list(i32::MAX, primes.clone()), false); // Maximum i32 value is not prime
        assert_eq!(is_prime_list(i32::MIN, primes), false); // Minimum i32 value is not prime
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
    fn is_prime_large_input_test() {
        // Test case for large input numbers
        let primes = generate_primes(1000);
        assert_eq!(is_prime_list(997, primes.clone()), true); // 997 is prime
        assert_eq!(is_prime_list(1001, primes), false); // 1001 is not prime
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
