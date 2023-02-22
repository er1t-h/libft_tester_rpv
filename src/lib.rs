#![allow(non_camel_case_types)]

#[cfg(test)]
mod test;

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
#[cfg(feature = "verbose")]
macro_rules! verbose {
	($($args: expr),+) => {
		println!($($args),+);
	};
}
#[allow(unused_macros)]
#[cfg(not(feature = "verbose"))]
macro_rules! verbose {
    ($($args: expr),+) => {};
}

#[allow(unused_imports)]
pub(crate) use assert_nzero;
#[allow(unused_imports)]
pub(crate) use assert_same_sign;
#[allow(unused_imports)]
pub(crate) use verbose;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
