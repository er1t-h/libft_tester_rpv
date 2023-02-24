use std::{cmp::Ordering, ffi::CString};

fn rotx(index: usize, c: u8) -> u8 {
    if c.is_ascii_uppercase() {
        (((c - b'A') as usize + index) % 26) as u8 + b'A'
    } else if c.is_ascii_lowercase() {
        (((c - b'a') as usize + index) % 26) as u8 + b'a'
    } else {
        c
    }
}

fn rotone(_index: usize, c: u8) -> u8 {
    if c.is_ascii_uppercase() {
        ((c - b'A' + 1) % 26) + b'A'
    } else if c.is_ascii_lowercase() {
        ((c - b'a' + 1) % 26) + b'a'
    } else {
        c
    }
}

fn to_num(index: usize, c: u8) -> u8 {
    if c.is_ascii_uppercase() {
        (((c - b'A') as usize + index) % 10) as u8 + b'0'
    } else if c.is_ascii_lowercase() {
        (((c - b'a') as usize + index) % 10) as u8 + b'0'
    } else {
        c
    }
}

macro_rules! test {
	($name: ident, $str: expr, $func: ident) => {
		crate::fork_test!{
			#![rusty_fork(timeout_ms = 1000)]

			#[test]
			fn $name() {
				let cchars = CString::new($str).unwrap();
				let user_ret = unsafe { crate::ft_strmapi(cchars.as_ptr(), Some(crate::$func)) };
				let expected = cchars.as_bytes().iter().enumerate().map(|(index, value)| $func(index, *value));
				let bytes = unsafe { std::slice::from_raw_parts(user_ret as *mut u8, $str.len()) };
				let expected_as_vec = expected.collect::<Vec<u8>>();
				assert_eq!(expected_as_vec.as_slice().cmp(bytes), Ordering::Equal);
				unsafe { libc::free(user_ret as *mut libc::c_void) };
			}
		}
	};
}

test!(basic_rotone, "Bonjour", rotone);
test!(
    longer_rotone,
    "Un super test de qualite n'est-ce pas",
    rotone
);
test!(basic_rotx, "Bonjour", rotx);
test!(longer_rotx, "Un super test de qualite n'est-ce pas", rotx);
test!(basic_to_num, "Bonjour", to_num);
test!(
    longer_to_num,
    "Un super test de qualite n'est-ce pas",
    to_num
);

crate::fork_test! {
    #![rusty_fork(timeout_ms = 1000)]

    #[test]
    fn str_as_null() {
        unsafe { crate::ft_strmapi(std::ptr::null(), Some(crate::rotx)) };
    }

    #[test]
    fn fn_as_null() {
        let expected = CString::new("Bonjour").unwrap();
        let ret = unsafe { crate::ft_strmapi(expected.as_ptr(), None) };
        if ret.is_null() {
            crate::verbose!("User chose to return NULL");
            return;
        }
        let content = unsafe { std::slice::from_raw_parts(ret as *mut u8, 8) };
        if content == expected.as_bytes() {
            unsafe { libc::free(ret as *mut libc::c_void) };
            crate::verbose!("User chose to return a copy of str");
            return;
        }

        // If you go through here, you handled passing NULL in a way I didn't anticipate.
        // If you can explain it clearly, you may ignore this failing test.
        // However, if you let your code crash with this test
        // because you didn't handle having NULL as an argument, you should get
        // a Crash flag.
        panic!("Handled passing NULL to strjoin in a strange way");
    }

    #[test]
    fn both_null() {
        unsafe { crate::ft_strmapi(std::ptr::null_mut(), None) };
    }
}
