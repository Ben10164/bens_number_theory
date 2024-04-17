#[cfg(test)]
mod is_prime_list_tests {
    use bens_number_theory::primes::{generate_primes, is_prime_list};

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
    use bens_number_theory::primes::is_prime_lazy;

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
    use bens_number_theory::primes::is_prime;

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
        assert_eq!(generate_primes(1000000).len(), 78498); // There are 78498 primes up to 1000000
    }
}

#[cfg(test)]
mod is_mersenne_prime_tests {
    use bens_number_theory::primes::is_mersenne_prime;

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
