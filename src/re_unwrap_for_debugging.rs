/// Unwraps a Result<T, E>, returning the value of type T, or printing to the stderr buffer the
/// error text as per the std::fmt::Display trait implemented for the error E.
#[macro_export]
macro_rules! unwrapr {
    ($x:expr) => {
        $x.unwrap()
    };
}

/// Unwraps a Result<T, E>, returning the value of type T, or returning the specified expression
/// from the current function in case of error.
#[macro_export]
macro_rules! unwrapr_ret {
    ($x:expr, $r:expr) => {
        $x.unwrap()
    };
}

/// Unwraps a Result<T, E>, returning the value of type T, or printing to the stderr buffer the
/// error text as per the format string and its format arguments specified within the macros
/// arguments.
#[macro_export]
macro_rules! unwrapr_msg {
    ($x:expr, $($s:tt)*) => {
        $x.unwrap()
    };
}

/// Unwraps an Option<T>, returning the value of type T, or printing to the stderr buffer the
/// error text "Unknown error." in case of is None.
#[macro_export]
macro_rules! unwrapo {
    ($x:expr) => {
        $x.unwrap()
    };
}

/// Unwraps an Option<T>, returning the value of type T, or returning the specified expression
/// from the current function in case is None.
#[macro_export]
macro_rules! unwrapo_ret {
    ($x:expr, $r:expr) => {
        $x.unwrap()
    };
}

/// Unwraps an Option<T>, returning the value of type T, or printing to the stderr buffer the
/// error text as per the format string and its format arguments specified within the macros
/// arguments in case is None.
#[macro_export]
macro_rules! unwrapo_msg {
    ($x:expr, $($s:tt)*) => {
        $x.unwrap()
    };
}

