/// Assume two values are not equal.
///
/// * When true, return `Ok(true)`.
///
/// * When false, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate assertable; fn main() {
/// let x = assume_ne!(1, 2);
/// //-> Ok(true)
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertable; fn main() {
/// let x = assume_ne!(1, 1);
/// //-> Err("assume_ne left:1 right:1")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assume_ne {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val != right_val) {
                    Ok(true)
                } else {
                    Err(format!("assumption failed: `assume_ne(left, right)`\n  left: `{:?}`\n right: `{:?}`", $left, $right))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val != right_val) {
                    Ok(true)
                } else {
                    Err($($arg)+)
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assume_ne_x_arity_2_success() {
        let a = 1;
        let b = 2;
        let x = assume_ne!(a, b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_ne_x_arity_2_failure() {
        let a = 1;
        let b = 1;
        let x = assume_ne!(a, b);
        assert_eq!(
            x.unwrap_err(),
            "assumption failed: `assume_ne(left, right)`\n  left: `1`\n right: `1`"
        );
    }

    #[test]
    fn test_assume_ne_x_arity_3_success() {
        let a = 1;
        let b = 2;
        let x = assume_ne!(a, b, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_ne_x_arity_3_failure() {
        let a = 1;
        let b = 1;
        let x = assume_ne!(a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
