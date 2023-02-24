//!
//! Tests for memset
//!
//! Not testing NULL because it's undefined, so not crashing would be as
//! justifiable as crashing
//!

macro_rules! test {
	($name: ident, $buffer_size: expr, $char_to_write: expr, $number_to_write: expr) => {
		crate::fork_test! {
			#[test]
			fn $name() {
				let mut buffer_user = [1_u8; $buffer_size];
				let mut buffer_libc = [1_u8; $buffer_size];

				unsafe { crate::ft_memset(buffer_user.as_mut_ptr() as *mut libc::c_void, $char_to_write as i32, $number_to_write) };
				unsafe { libc::memset(buffer_libc.as_mut_ptr() as *mut libc::c_void, $char_to_write as i32, $number_to_write) };
				assert_eq!(buffer_libc, buffer_user);
			}
		}
	};
	($name: ident, $buffer_size: expr, $char_to_write: expr) => {
		test!($name, $buffer_size, $char_to_write, $buffer_size);
	};
}

test!(basic, 1000, 'w');
test!(replace_half, 1000, '5', 500);
test!(no_replace, 1000, 'v', 0);
