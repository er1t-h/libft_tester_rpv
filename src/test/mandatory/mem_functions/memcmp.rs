use libc::c_void;
use crate::assert_same_sign;

macro_rules! test {
	($name: ident, $size: expr, $buff1: expr, $buff2: expr) => {
		crate::fork_test! {
			#[test]
			fn $name() {
				let first_buffer = $buff1;
				let second_buffer = $buff2;
				let user_ret = unsafe {
					crate::ft_memcmp(first_buffer.as_ptr() as *const c_void, second_buffer.as_ptr() as *const c_void, 4 * $size)
				};
				let libc_ret = unsafe {
					libc::memcmp(first_buffer.as_ptr() as *const c_void, second_buffer.as_ptr() as *const c_void, 4 * $size)
				};
				assert_same_sign!(libc_ret, user_ret);
			}
		}
	};
	($name: ident, $size: expr, $buff1: expr) => {
		test!($name, $size, $buff1, $buff1);
	};
}

test!(basic, 10, [11037, 564, 12, 4674, 45748, 0, 112, 15, 564, 546]);
test!(mismatch_after_len, 2, [11037, 564, 12, 4674, 45748, 0, 112, 15, 564, 546], [11037, 564, 145, 4674, 45748, 0, 112, 15, 564, 546]);
test!(mismatch_in_len, 10, [11037, 564, 12, 4674, 45748, 0, 112, 15, 564, 546], [11037, 564, 145, 4674, 45748, 0, 112, 15, 564, 546]);
test!(compare_zero, 0, [2147483647, 564, 12, 4674, 45748, 0, 112, 15, 564, 546], [0, 564, 145, 4674, 45748, 0, 112, 15, 564, 546]);
