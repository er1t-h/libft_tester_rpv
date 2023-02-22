#![allow(non_camel_case_types)]

#[cfg(test)]
mod test;

macro_rules! assert_same_sign {
	($item1: expr, $item2: expr) => {
		assert!(
			($item1 > 0 && $item2 > 0)
			|| ($item1 == 0 && $item2 == 0)
			|| ($item1 < 0 && $item2 < 0),
			"\nError in assert_same_sign:\n\t left: `{}`\n\tright: `{}`\n",
			$lhs,
			$rhs
		);
	};
}

macro_rules! assert_nzero {
	($lhs: expr, $rhs: expr) => {
		assert!(
			($lhs != 0 && $rhs != 0)
			|| ($lhs == 0 && $rhs == 0),
			"\nError in assert_nzero:\n\t left: `{}`\n\tright: `{}`\n",
			$lhs,
			$rhs
		);
	};
}

#[cfg(feature = "verbose")]
macro_rules! verbose {
	($($args: expr),+) => {
		println!($($args),+);
	};
}
#[cfg(not(feature = "verbose"))]
macro_rules! verbose {
	($($args: expr),+) => {};
}

pub(crate) use assert_same_sign;
pub(crate) use assert_nzero;
pub(crate) use verbose;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
