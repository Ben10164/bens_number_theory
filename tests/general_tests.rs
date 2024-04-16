#[cfg(test)]
mod general_tests {
    use bens_number_theory::*;

    #[test]
    fn divisors_test() {
        assert_eq!(divisors(10), vec![1, 2, 5, 10]);
        assert_eq!(divisors(20), vec![1, 2, 4, 5, 10, 20]);
    }
}
