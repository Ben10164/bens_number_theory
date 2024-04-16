#[cfg(test)]
mod factorial_tests {
    use bens_number_theory::factorials::factorial;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }
}

#[cfg(test)]
mod factorial_list_tests {
    use bens_number_theory::factorials::factorial_list;

    #[test]
    fn test_factorial_list() {
        let factorials: Vec<u128> = factorial_list(10);
        assert_eq!(
            factorials,
            [1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800]
        )
    }
}
