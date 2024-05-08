#[cfg(test)]
mod is_prime_tests {
    use bens_number_theory::primes::{generate_primes, is_prime, is_prime_lazy, is_prime_list};

    #[test]
    fn test_prime_numbers_lazy() {
        assert!(is_prime_lazy(2));
        assert!(is_prime_lazy(3));
        assert!(is_prime_lazy(5));
        assert!(is_prime_lazy(7));
        assert!(is_prime_lazy(13));
    }

    #[test]
    fn test_non_prime_numbers_lazy() {
        assert!(!is_prime_lazy(1));
        assert!(!is_prime_lazy(4));
        assert!(!is_prime_lazy(6));
        assert!(!is_prime_lazy(8));
        assert!(!is_prime_lazy(10));
        assert!(!is_prime_lazy(12));
        assert!(!is_prime_lazy(14));
    }

    #[test]
    fn test_non_prime_numbers() {
        // Test some non-prime numbers
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
        assert!(!is_prime(12));
        assert!(!is_prime(14));
        assert!(!is_prime(15));
        assert!(!is_prime(16));
    }

    #[test]
    fn test_prime_numbers() {
        // Test some prime numbers
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
        assert!(is_prime(17));
        assert!(is_prime(19));
        assert!(is_prime(23));
    }

    #[test]
    fn test_negative_numbers() {
        // Test negative numbers
        assert!(!is_prime(-2));
        assert!(!is_prime(-3));
        assert!(!is_prime(-5));
        assert!(!is_prime(-7));
    }

    #[test]
    fn test_zero_and_one() {
        // Test zero and one
        assert!(!is_prime(0));
        assert!(!is_prime(1));
    }

    #[test]
    fn is_prime_large_test() {
        // Test case for checking prime numbers up to a large limit
        let primes: Vec<i32> = generate_primes(100);
        assert!(is_prime_list(97, primes.clone())); // 97 is prime
        assert!(!is_prime_list(99, primes.clone())); // 99 is not prime
        assert!(is_prime_list(101, primes)); // 101 is prime
    }

    #[test]
    fn is_prime_negative_numbers_test() {
        // Test case for negative numbers
        let primes: Vec<i32> = vec![2, 3, 5, 7];
        assert!(!is_prime_list(-7, primes.clone())); // -7 is not prime
        assert!(!is_prime_list(-11, primes)); // -11 is not prime
    }

    #[test]
    fn is_prime_edge_case_test() {
        // Test case for edge cases of prime number detection
        let primes: Vec<i32> = vec![2, 3, 5, 7];
        assert!(!is_prime_list(i32::MAX, primes.clone())); // Maximum i32 value is not prime
        assert!(!is_prime_list(i32::MIN, primes)); // Minimum i32 value is not prime
    }

    #[test]
    fn is_prime_large_input_test() {
        // Test case for large input numbers
        let primes: Vec<i32> = generate_primes(1000);
        assert!(is_prime_list(997, primes.clone())); // 997 is prime
        assert!(!is_prime_list(1001, primes)); // 1001 is not prime
    }
}

#[cfg(test)]
mod generate_primes_tests {
    use bens_number_theory::primes::generate_primes;

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
        assert_eq!(generate_primes(100000).len(), 9592); // There are 9592 primes up to 100000
    }
}

#[cfg(test)]
mod is_mersenne_prime_tests {
    use bens_number_theory::primes::is_mersenne_prime;

    /// True

    #[test]
    fn mersenne_primes_tests1() {
        assert!(is_mersenne_prime((2_u128.pow(2)) - 1));
    }

    #[test]
    fn mersenne_primes_tests2() {
        assert!(is_mersenne_prime((2_u128.pow(3)) - 1));
    }

    #[test]
    fn mersenne_primes_tests3() {
        assert!(is_mersenne_prime((2_u128.pow(5)) - 1));
    }

    #[test]
    fn mersenne_primes_tests4() {
        assert!(is_mersenne_prime((2_u128.pow(7)) - 1));
    }

    #[test]
    fn mersenne_primes_tests5() {
        assert!(is_mersenne_prime((2_u128.pow(13)) - 1));
    }

    #[test]
    fn mersenne_primes_tests6() {
        assert!(is_mersenne_prime((2_u128.pow(17)) - 1));
    }

    #[test]
    fn mersenne_primes_tests7() {
        assert!(is_mersenne_prime((2_u128.pow(19)) - 1));
    }

    /// False

    #[test]
    fn non_mersenne_primes_tests1() {
        assert!(!is_mersenne_prime((2_u128.pow(4)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests2() {
        assert!(!is_mersenne_prime((2_u128.pow(6)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests3() {
        assert!(!is_mersenne_prime((2_u128.pow(8)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests4() {
        assert!(!is_mersenne_prime((2_u128.pow(9)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests5() {
        assert!(!is_mersenne_prime((2_u128.pow(10)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests6() {
        assert!(!is_mersenne_prime((2_u128.pow(11)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests7() {
        assert!(!is_mersenne_prime((2_u128.pow(12)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests8() {
        assert!(!is_mersenne_prime((2_u128.pow(14)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests9() {
        assert!(!is_mersenne_prime((2_u128.pow(15)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests10() {
        assert!(!is_mersenne_prime((2_u128.pow(16)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests11() {
        assert!(!is_mersenne_prime((2_u128.pow(18)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests12() {
        assert!(!is_mersenne_prime((2_u128.pow(20)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests13() {
        assert!(!is_mersenne_prime((2_u128.pow(21)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests14() {
        assert!(!is_mersenne_prime((2_u128.pow(22)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests15() {
        assert!(!is_mersenne_prime((2_u128.pow(23)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests16() {
        assert!(!is_mersenne_prime((2_u128.pow(24)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests17() {
        assert!(!is_mersenne_prime((2_u128.pow(25)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests18() {
        assert!(!is_mersenne_prime((2_u128.pow(26)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests19() {
        assert!(!is_mersenne_prime((2_u128.pow(27)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests20() {
        assert!(!is_mersenne_prime((2_u128.pow(28)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests21() {
        assert!(!is_mersenne_prime((2_u128.pow(29)) - 1));
    }

    #[test]
    fn non_mersenne_primes_tests22() {
        assert!(!is_mersenne_prime((2_u128.pow(30)) - 1));
    }
}
