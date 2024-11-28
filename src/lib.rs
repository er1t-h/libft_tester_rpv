#![allow(non_camel_case_types)]

#[allow(dead_code)]
mod generate;
#[allow(dead_code)]
mod libft;
#[cfg(test)]
mod test;
#[allow(dead_code)]
mod utils;

use std::sync::LazyLock;

pub use libft::LIBRARY;
const DEFAULT_RANDOM_REPEAT_NUMBER: usize = 10;
pub static RANDOM_REPEAT_NUMBER: LazyLock<usize> = LazyLock::new(|| {
    std::env::var("LIBFT_TESTER_RANDOM_REPEAT")
        .map(|x| x.parse::<usize>())
        .unwrap_or(Ok(DEFAULT_RANDOM_REPEAT_NUMBER))
        .unwrap_or(DEFAULT_RANDOM_REPEAT_NUMBER)
});

#[allow(unused_macros)]
macro_rules! assert_same_sign {
    ($lhs: expr, $rhs: expr) => {
        assert!(
            ($lhs > 0 && $rhs > 0) || ($lhs == 0 && $rhs == 0) || ($lhs < 0 && $rhs < 0),
            "\nError in assert_same_sign:\n\t left: `{}`\n\tright: `{}`\n",
            $lhs,
            $rhs
        );
    };
    ($lhs: expr, $rhs: expr, $($rest: tt)+) => {
        assert!(
            ($lhs > 0 && $rhs > 0) || ($lhs == 0 && $rhs == 0) || ($lhs < 0 && $rhs < 0), $($rest)+
        );
    };
}

#[allow(unused_macros)]
macro_rules! assert_nzero {
    ($lhs: expr, $rhs: expr) => {
        assert!(
            ($lhs != 0 && $rhs != 0) || ($lhs == 0 && $rhs == 0),
            "\nError in assert_nzero:\n\t left: `{}`\n\tright: `{}`\n",
            $lhs,
            $rhs
        );
    };
}

#[allow(unused_macros)]
#[cfg(feature = "fork")]
macro_rules! fork_test {
    (#![rusty_fork(timeout_ms = $timeout: expr)]
     $(
        $(#[$meta:meta])*
        fn $test_name:ident($($param_name: tt: $param_type: ty),*) $body:block
    )*) => {
        rusty_fork::rusty_fork_test!{
            #![rusty_fork(timeout_ms = $timeout)]
            $(
                $(#[$meta])*
                fn $test_name($($param_name: $param_type),*) $body
            )*
        }
    };
    ($(
        $(#[$meta:meta])*
        fn $test_name:ident($($param_name: tt: $param_type: ty),*) $body:block
    )*) => {
        rusty_fork::rusty_fork_test!{
            #![rusty_fork(timeout_ms = 30000)]
            $(
                $(#[$meta])*
                fn $test_name($($param_name: $param_type),*) $body
            )*
        }
    };
}
#[allow(unused_macros)]
#[cfg(not(feature = "fork"))]
macro_rules! fork_test {
    (#![rusty_fork(timeout_ms = $timeout: expr)]
     $(
        $(#[$meta:meta])*
        fn $test_name:ident($($param_name: tt: $param_type: ty),*) $body:block
    )*) => {
        $(
            $(#[$meta])*
            fn $test_name($($param_name: $param_type),*) {
                $body
            }
        )*
    };
    ($(
        $(#[$meta:meta])*
        fn $test_name:ident($($param_name: tt: $param_type: ty),*) $body:block
    )*) => { $(
            $(#[$meta])*
            fn $test_name($($param_name: $param_type),*) {
                $body
            }
        )*
    };
}

#[allow(unused_imports)]
pub(crate) use assert_nzero;
#[allow(unused_imports)]
pub(crate) use assert_same_sign;
#[allow(unused_imports)]
pub(crate) use fork_test;
