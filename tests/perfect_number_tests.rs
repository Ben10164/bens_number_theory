#[cfg(test)]
mod general_tests {
    use bens_number_theory::perfect_numbers::{divisors, is_perfect_number};

    #[test]
    fn divisors_test() {
        assert_eq!(divisors(10), vec![1, 2, 5, 10]);
        assert_eq!(divisors(20), vec![1, 2, 4, 5, 10, 20]);
    }

    #[test]
    fn perfect_number_test() {
        assert!(is_perfect_number(6));
        assert!(!is_perfect_number(7));
        assert!(is_perfect_number(28));
        assert!(!is_perfect_number(29));
        assert!(is_perfect_number(8128));
        assert!(!is_perfect_number(8130));
    }
}
