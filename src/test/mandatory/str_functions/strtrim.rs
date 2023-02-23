use std::ffi::CString;
use crate::verbose;

macro_rules! test {
	($name: ident, $str: expr, $set: expr) => {
		crate::fork_test!{
			#[test]
			fn $name() {
				let str = CString::new($str).unwrap();
				let set = CString::new($set).unwrap();
				let user_ret = unsafe {
					crate::ft_strtrim(str.as_ptr(), set.as_ptr())
				};
				let trimmed_ret = CString::new($str.trim_matches(|x| $set.contains(x))).unwrap();
				let content = unsafe { std::slice::from_raw_parts(user_ret as *mut u8, trimmed_ret.as_bytes_with_nul().len()) };
				assert_eq!(content, trimmed_ret.as_bytes_with_nul());
				unsafe { libc::free(user_ret as *mut libc::c_void) };
			}
		}
	};
}

test!(basic, "Bonjour", "Br");
test!(trim_all, "Bonjour", "nojurB");
test!(utf8, "🀄Super🀄", "🀄");
test!(trim_none, "🀄Super🀄", "BOFPASTRP");

crate::fork_test!{
	#[test]
	fn first_null() {
		let set = CString::new("Super").unwrap();
		let user_ret = unsafe {
			crate::ft_strtrim(std::ptr::null(), set.as_ptr())
		};
		if user_ret.is_null() {
			verbose!("User choosed to handle by returning NULL");
			return;
		}
		if unsafe { *user_ret } == 0 {
			unsafe {libc::free(user_ret as *mut libc::c_void)};
			verbose!("Creating an empty string");
			return;
		}

		// If you go through here, you handled passing NULL in a way I didn't anticipate.
		// If you can explain it clearly, you may ignore this failing test.
		// However, if you let your code crash with this test
		// because you didn't handle having NULL as an argument, you should get
		// a Crash flag.
		panic!("Handled passing NULL to strtrim in a strange way");
	}

	#[test]
	fn second_null() {
		let str = CString::new("Super").unwrap();
		let user_ret = unsafe {
			crate::ft_strtrim(str.as_ptr(), std::ptr::null())
		};
		if user_ret.is_null() {
			verbose!("User choosed to handle by returning NULL");
			return;
		}
		let content = unsafe { std::slice::from_raw_parts(user_ret as *mut u8, str.as_bytes_with_nul().len()) };
		if str.as_bytes_with_nul() == content {
			unsafe { libc::free(user_ret as *mut libc::c_void) };
			verbose!("Creating a copy");
			return;
		}

		// If you go through here, you handled passing NULL in a way I didn't anticipate.
		// If you can explain it clearly, you may ignore this failing test.
		// However, if you let your code crash with this test
		// because you didn't handle having NULL as an argument, you should get
		// a Crash flag.
		panic!("Handled passing NULL to strtrim in a strange way");
	}

	#[test]
	fn both_null() {
		let user_ret = unsafe {
			crate::ft_strtrim(std::ptr::null(), std::ptr::null())
		};
		if user_ret.is_null() {
			verbose!("User choosed to handle by returning NULL");
			return;
		}
		if unsafe { *user_ret } == 0 {
			unsafe {libc::free(user_ret as *mut libc::c_void)};
			verbose!("Creating an empty string");
			return;
		}

		// If you go through here, you handled passing NULL in a way I didn't anticipate.
		// If you can explain it clearly, you may ignore this failing test.
		// However, if you let your code crash with this test
		// because you didn't handle having NULL as an argument, you should get
		// a Crash flag.
		panic!("Handled passing NULL to strtrim in a strange way");
	}
}
