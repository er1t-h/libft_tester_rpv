use std::ffi::CString;
#[cfg(feature = "fork")]
use rusty_fork::rusty_fork_test;

macro_rules! test {
	($name: ident, $str: expr, $start: expr, $len: expr) => {
		crate::fork_test!{
			#[test]
			fn $name() {
				#[allow(unused_comparisons)]
				const NUMBER_OF_CHAR: usize = if $len > $str.len() - $start {$str.len() - $start} else {$len};
				let base_str = CString::new($str).unwrap();
				let result = unsafe {
					crate::ft_substr(base_str.as_ptr(), $start, $len)
				};
				let content = unsafe {
					std::slice::from_raw_parts(result as *mut u8, NUMBER_OF_CHAR + 1)
				};
				crate::verbose!("User ret: `{}`", std::str::from_utf8(content).unwrap());
				crate::verbose!("Expected: `{}`", &$str[$start..$start + NUMBER_OF_CHAR]);
				assert_eq!(&content[..NUMBER_OF_CHAR], &$str.as_bytes()[$start..$start + NUMBER_OF_CHAR]);
				assert_eq!(*content.last().unwrap(), 0);
				unsafe {libc::free(result as *mut libc::c_void)};
			}
		}
	};
}

test!(basic, "SuperTest", 1, 7);
test!(no_copy, "SuperTest", 1, 0);
// If this test failed, the user probably allocates $len without verifying that
// it needs that much character. I'd consider it false, but that's debatable.
test!(bigger_buffer, "SuperTest", 0, libc::size_t::MAX);

crate::fork_test! {
	#[test]
	fn start_after_end_of_string() {
		let base_str = CString::new("SuperTest").unwrap();
		let result = unsafe {
			crate::ft_substr(base_str.as_ptr(), 1000, 6)
		};
		if result.is_null() {
			crate::verbose!("User choose to handle passing a start higher than len \
							by returning NULL.");
			return;
		}
		if unsafe { *result } == 0 {
			unsafe {libc::free(result as *mut libc::c_void)};
			crate::verbose!("User choose to handle passing a start higher than len \
							by returning an empty string.");
			return;
		}

		// If you go through here, you handled passing NULL in a way I didn't anticipate.
		// If you can explain it clearly, you may ignore this failing test.
		// However, if you let your code crash with this test
		// because you didn't handle having NULL as an argument, you should get
		// a Crash flag.
		panic!("Handled passing start higher than len to substr in a strange way");
	}


	#[test]
	fn null() {
		let result = unsafe {
			crate::ft_substr(std::ptr::null(), 1000, 6)
		};
		if result.is_null() {
			crate::verbose!("User choose to handle passing a start higher than len \
							by returning NULL.");
			return;
		}

		// If you go through here, you handled passing NULL in a way I didn't anticipate.
		// If you can explain it clearly, you may ignore this failing test.
		// However, if you let your code crash with this test
		// because you didn't handle having NULL as an argument, you should get
		// a Crash flag.
		panic!("Handled passing NULL to substr in a strange way");
	}
}
