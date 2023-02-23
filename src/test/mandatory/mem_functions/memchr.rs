use libc::c_void;

macro_rules! test {
	($name: ident, $buffer: expr, $to_find: expr, $size: expr) => {
		crate::fork_test! {
			#[test]
			fn $name() {
				let buffer = $buffer;
				let user_ret = unsafe {
					crate::ft_memchr(buffer.as_ptr() as *const c_void, $to_find as i32, $size)
				};
				let libc_ret = unsafe {
					libc::memchr(buffer.as_ptr() as *const c_void, $to_find as i32, $size)
				};
				assert_eq!(user_ret, libc_ret);
			}
		}
	};
	($name: ident, $buffer: expr, $to_find: expr) => {
		test!($name, $buffer, $to_find, $buffer.len());
	}
}

test!(basic, b"Un\0Super\0Test\0Oui", 'O');
test!(zero_size, b"Un\0Super\0Test\0Oui", 'O', 0);
test!(no_match, b"Un\0Super\0Test\0Oui", '\n', 0);
test!(match_zero, b"Un\0Super\0Test\0Oui", '\0', 0);
