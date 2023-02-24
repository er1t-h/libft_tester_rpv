use libc::c_void;

macro_rules! test {
	($name: ident, $src: expr, $size: expr) => {
		crate::fork_test!{
			#[test]
			fn $name() {
				let src = $src;
				let mut user_buffer = [0_u8; $size];
				let mut libc_buffer = [0_u8; $size];
				let user_ret = unsafe { crate::ft_memcpy(user_buffer.as_mut_ptr() as *mut c_void, src.as_ptr() as *mut c_void, $size) };
				assert_eq!(user_ret, user_buffer.as_mut_ptr() as *mut c_void, "Return value and dest do not match");
				unsafe { libc::memcpy(libc_buffer.as_mut_ptr() as *mut c_void, src.as_ptr() as *mut c_void, $size) };
				assert_eq!(user_buffer, libc_buffer);
			}
		}
	};
	($name: ident, $src: expr) => {
		test!($name, $src, $src.len());
	};
}

test!(basic, "SuperTestDeQualité");
test!(partial_copy, "SuperTestDeQualitéfjwegpj", 10);
test!(no_copy, "UnSuperTest", 0);
