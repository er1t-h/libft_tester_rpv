use libc::c_void;
use pretty_assertions::assert_eq;

macro_rules! test {
	($name: ident, $buffer_total_size: expr, $buffer_initial_content: expr, $offset_dest: expr, $offset_src: expr, $size_to_move: expr) => {
		#[test]
		fn $name() {
			let mut buffer_user = [0_u8; $buffer_total_size];
			let mut buffer_libc = [0_u8; $buffer_total_size];
			let src = $buffer_initial_content;
			unsafe {
				libc::memcpy(buffer_user.as_mut_ptr() as *mut c_void, src.as_ptr() as *const c_void, src.len());
				libc::memcpy(buffer_libc.as_mut_ptr() as *mut c_void, src.as_ptr() as *const c_void, src.len());
			}
			let destination = unsafe {
				buffer_user.as_mut_ptr().add($offset_dest) as *mut c_void
			};
			let user_ret = unsafe {
				crate::ft_memmove(destination, buffer_user.as_ptr().add($offset_src) as *const c_void, $size_to_move)
			};
			unsafe {
				libc::memmove(buffer_libc.as_mut_ptr().add($offset_dest) as *mut c_void, buffer_user.as_ptr().add($offset_src) as *const c_void, $size_to_move);
			};
			assert_eq!(user_ret, destination, "The return value doesn't match dest.");
			assert_eq!(buffer_user, buffer_libc, "The content of the buffers differs");
		}
	};
	($name: ident, $buffer_total_size: expr, $buffer_initial_content: expr, $offset_dest: expr, $offset_src: expr) => {
		test!($name, $buffer_total_size, $buffer_initial_content, $offset_dest, $offset_src, $buffer_initial_content.len());
	}
}

test!(basic, 50, b"Bonjour mes chers amis !", 25, 0);
test!(overlapping_at_begin, 50, b"Bonjour mes chers amis !", 0, 10);
test!(overlapping_at_end, 50, b"Bonjour mes chers amis !", 10, 0);
test!(no_offset, 35, b"Bonjour mes chers amis !", 0, 0);
