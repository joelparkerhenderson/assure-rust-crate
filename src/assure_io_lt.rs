/// Assure one value is less than another value.
///
/// * When true, return `Ok(true)`.
///
/// * When false, return `Ok(false)`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate assertable; fn main() {
/// let x = assure_io_lt!(1, 2);
/// //-> Ok(true)
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertable; fn main() {
/// let x = assure_io_lt!(2, 1);
/// //-> Ok(false)
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_io_lt {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val < right_val) {
                    Ok(true)
                } else {
                    Ok(false)                    
                }
            }
        }
    } as Result<bool, String>);
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val < right_val) {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    } as Result<bool, String>);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_io_lt_x_arity_2_success() {
        let a = 1;
        let b = 2;
        let x = assure_io_lt!(a, b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_io_lt_x_arity_2_failure() {
        let a = 2;
        let b = 1;
        let x = assure_io_lt!(a, b);
        assert_eq!(
            x.unwrap(),
            false
            //x.unwrap_err().get_ref().unwrap().to_string(),
            //"assure_io_lt left:2 right:1"
        );
    }

    #[test]
    fn test_assure_io_lt_x_arity_3_success() {
        let a = 1;
        let b = 2;
        let x = assure_io_lt!(a, b, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_io_lt_x_arity_3_failure() {
        let a = 2;
        let b = 1;
        let x = assure_io_lt!(a, b, "message");
        assert_eq!(
            x.unwrap(),
            false
            //x.unwrap_err().get_ref().unwrap().to_string(),
            //"message"
        );
    }

}
