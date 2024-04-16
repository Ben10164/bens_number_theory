#[cfg(test)]
mod general_tests {
    use bens_number_theory::*;

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
