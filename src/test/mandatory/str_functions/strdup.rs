use std::{ffi::CString, slice::from_raw_parts};
#[cfg(feature = "fork")]
use rusty_fork::rusty_fork_test;

macro_rules! test {
	($name: ident, $str: expr) => {
		crate::fork_test!{
			#[test]
			fn $name() {
				let str = CString::new($str).unwrap();
				let user_ret = unsafe {
					crate::ft_strdup(str.as_ptr())
				};
				let content = unsafe { from_raw_parts(user_ret as *mut u8, str.as_bytes_with_nul().len()) };
				assert_eq!(str.as_bytes_with_nul(), content);
				unsafe {libc::free(user_ret as *mut libc::c_void)};
			}
		}
	};
}

test!(basic, "SuperTest");
test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");
