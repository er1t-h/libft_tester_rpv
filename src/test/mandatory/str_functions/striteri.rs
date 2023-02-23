use std::cmp::Ordering;

fn rotx_in_place(index: usize, c: u8) -> u8 {
	if c.is_ascii_uppercase() {
		(((c - b'A') as usize + index) % 26) as u8 + b'A'
	} else if c.is_ascii_lowercase() {
		(((c - b'a') as usize + index) % 26) as u8 + b'a'
	} else {
		c
	}
}

fn rotone_in_place(_index: usize, c: u8) -> u8 {
	if c.is_ascii_uppercase() {
		((c - b'A' + 1) % 26) + b'A'
	} else if c.is_ascii_lowercase() {
		((c - b'a' + 1) % 26) + b'a'
	} else {
		c
	}
}

fn to_num_in_place(index: usize, c: u8) -> u8 {
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
				let mut cchars = $str.map(|c| c as libc::c_char);
				unsafe { crate::ft_striteri(cchars.as_mut_ptr(), Some(crate::$func)) }
				let expected = $str.iter().enumerate().map(|(index, value)| $func(index, *value));
				let bytes = cchars.map(|x| x as u8);
				assert_eq!(expected.cmp(bytes), Ordering::Equal);
			}
		}
	};
}

test!(basic_rotone, b"Bonjour\0", rotone_in_place);
test!(longer_rotone, b"Un super test de qualite n'est-ce pas\0", rotone_in_place);
test!(basic_rotx, b"Bonjour\0", rotx_in_place);
test!(longer_rotx, b"Un super test de qualite n'est-ce pas\0", rotx_in_place);
test!(basic_to_num, b"Bonjour\0", to_num_in_place);
test!(longer_to_num, b"Un super test de qualite n'est-ce pas\0", to_num_in_place);

crate::fork_test!{
	#![rusty_fork(timeout_ms = 1000)]

	#[test]
	fn str_as_null() {
		unsafe { crate::ft_striteri(std::ptr::null_mut(), Some(crate::rotx_in_place)) }
	}

	#[test]
	fn fn_as_null() {
		let expected = b"Bonjour\0".map(|c| c as libc::c_char);
		let mut cchars = expected.clone();
		unsafe { crate::ft_striteri(cchars.as_mut_ptr(), None) }
		assert_eq!(cchars, expected, "String was changed but function was NULL");
	}

	#[test]
	fn both_null() {
		unsafe { crate::ft_striteri(std::ptr::null_mut(), None) }
	}
}
