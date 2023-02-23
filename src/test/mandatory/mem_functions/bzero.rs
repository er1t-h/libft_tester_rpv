//!
//! Tests for bzero
//!
//! I used libc::explicit_bzero because the libc crate doesn't provide a bzero.
//! It works exactly the same way. The only difference is that it guarantees the
//! execution of the function, even if optimizer would judge it pointless.
//!
//! Not testing NULL because it's undefined, so not crashing would be as
//! justifiable as crashing
//!


macro_rules! test {
	($name: ident, $buffer_size: expr, $to_write: expr) => {
		#[test]
		fn $name() {
			let mut buffer_user = [1_u8; $buffer_size];
			let mut buffer_libc = [1_u8; $buffer_size];

			unsafe { crate::ft_bzero(buffer_user.as_mut_ptr() as *mut libc::c_void, $to_write) };
			unsafe { libc::explicit_bzero(buffer_libc.as_mut_ptr() as *mut libc::c_void, $to_write) };
			assert_eq!(buffer_libc, buffer_user);
		}
	};
	($name: ident, $buffer_size: expr) => {
		test!($name, $buffer_size, $buffer_size);
	};
}

test!(basic, 1000);
test!(replace_half, 1000, 500);
test!(no_replace, 1000, 0);
