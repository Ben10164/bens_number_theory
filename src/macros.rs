/// Read in input from stdin
///
/// This function takes a variable `$out` which will be assigned the
/// input from stdin parsed as type `$type`
///
/// # Arguments
///
/// * `out` - The variable to be set.
/// * `type` - The desired type of the variable to return.
///
/// # Returns
///
/// Nothing, but sets the value of `out`
///
/// # Examples
///
/// ```
/// use bens_number_theory::read;
/// read!(input_string as String);
/// println!("Input: {}", input_string);
/// ```
#[macro_export]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin()
            .read_line(&mut inner)
            .expect("Failed to read from stdin");
        let $out = inner
            .trim()
            .parse::<$type>()
            .expect("Failed to parse input");
    };
}
