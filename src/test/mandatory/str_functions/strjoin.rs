use crate::verbose;
use std::{ffi::CString, slice::from_raw_parts};

macro_rules! test {
	($name: ident, $s1: expr, $s2: expr) => {
		crate::fork_test!{
			#[test]
			fn $name() {
				let s1 = CString::new($s1).unwrap();
				let s2 = CString::new($s2).unwrap();
				let user_ret = unsafe {
					crate::ft_strjoin(s1.as_ptr(), s2.as_ptr())
				};
				let content = unsafe { from_raw_parts(user_ret as *mut u8, s1.as_bytes().len() + s2.as_bytes_with_nul().len()) };
				assert_eq!([s1.as_bytes(), s2.as_bytes_with_nul()].concat(), content);
				unsafe {libc::free(user_ret as *mut libc::c_void)};
			}
		}
	};
}

test!(basic, "Salut ", "toi");

test!(basic_2, "Bon on continue les tests", " en fait");
test!(first_empty, "", " en fait");
test!(second_empty, "hey", "");
test!(both_empty, "", "");

crate::fork_test! {
    #[test]
    fn first_null() {
        let s2 = CString::new("anything").unwrap();
        let user_ret = unsafe {
            crate::ft_strjoin(std::ptr::null(), s2.as_ptr())
        };
        if user_ret.is_null() {
            verbose!("User choosed to handle by returning NULL");
            return;
        }
        let content = unsafe { from_raw_parts(user_ret as *mut u8, s2.as_bytes_with_nul().len()) };
        if s2.as_bytes_with_nul() == content {
            unsafe {libc::free(user_ret as *mut libc::c_void)};
            verbose!("User choosed to handle by allocating only s2");
            return;
        }
        // If you go through here, you handled passing NULL in a way I didn't anticipate.
        // If you can explain it clearly, you may ignore this failing test.
        // However, if you let your code crash with this test
        // because you didn't handle having NULL as an argument, you should get
        // a Crash flag.
        panic!("Handled passing NULL to strjoin in a strange way");
    }
}

crate::fork_test! {
    #[test]
    fn second_null() {
        let s1 = CString::new("anything").unwrap();
        let user_ret = unsafe {
            crate::ft_strjoin(s1.as_ptr(), std::ptr::null())
        };
        if user_ret.is_null() {
            verbose!("User choosed to handle by returning NULL");
            return;
        }
        let content = unsafe { from_raw_parts(user_ret as *mut u8, s1.as_bytes_with_nul().len()) };
        if s1.as_bytes_with_nul() == content {
            unsafe {libc::free(user_ret as *mut libc::c_void)};
            verbose!("User choosed to handle by allocating only s1");
            return;
        }
        // If you go through here, you handled passing NULL in a way I didn't anticipate.
        // If you can explain it clearly, you may ignore this failing test.
        // However, if you let your code crash with this test
        // because you didn't handle having NULL as an argument, you should get
        // a Crash flag.
        panic!("Handled passing NULL to strjoin in a strange way");
    }
}

crate::fork_test! {
    #[test]
    fn both_null() {
        let user_ret = unsafe {
            crate::ft_strjoin(std::ptr::null(), std::ptr::null())
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
        panic!("Handled passing NULL to strjoin in a strange way");
    }
}
