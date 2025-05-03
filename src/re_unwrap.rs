/// Unwraps a Result<T, E>, returning the value of type T, or printing to the stderr buffer the
/// error text as per the std::fmt::Display trait implemented for the error E.
#[macro_export]
macro_rules! unwrapr {
    ($x:expr) => {
        match $x {
            Ok(q) => q,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(-1);
            },
        }
    };
}

/// Unwraps a Result<T, E>, returning the value of type T, or returning the specified expression
/// from the current function in case of error.
#[macro_export]
macro_rules! unwrapr_ret {
    ($x:expr, $r:expr) => {
        match $x {
            Ok(q) => q,
            Err(_) => return $r,
        }
    };
}

/// Unwraps a Result<T, E>, returning the value of type T, or printing to the stderr buffer the
/// error text as per the format string and its format arguments specified within the macros
/// arguments.
#[macro_export]
macro_rules! unwrapr_msg {
    ($x:expr, $($s:tt)*) => {
        match $x {
            Ok(q) => q,
            Err(_) => {
                eprintln!("Error: {}", format!($($s)*));
                std::process::exit(-1);
            },
        }
    };
}

/// Unwraps an Option<T>, returning the value of type T, or printing to the stderr buffer the
/// error text "Unknown error." in case of is None.
#[macro_export]
macro_rules! unwrapo {
    ($x:expr) => {
        match $x {
            Some(q) => q,
            None => {
                eprintln!("Error: Unknown error.");
                std::process::exit(-1);
            },
        }
    };
}

/// Unwraps an Option<T>, returning the value of type T, or returning the specified expression
/// from the current function in case is None.
#[macro_export]
macro_rules! unwrapo_ret {
    ($x:expr, $r:expr) => {
        match $x {
            Some(q) => q,
            None => return $r,
        }
    };
}

/// Unwraps an Option<T>, returning the value of type T, or printing to the stderr buffer the
/// error text as per the format string and its format arguments specified within the macros
/// arguments in case is None.
#[macro_export]
macro_rules! unwrapo_msg {
    ($x:expr, $($s:tt)*) => {
        match $x {
            Some(q) => q,
            None => {
                eprintln!("Error: {}", format!($($s)*));
                std::process::exit(-1);
            },
        }
    };
}

