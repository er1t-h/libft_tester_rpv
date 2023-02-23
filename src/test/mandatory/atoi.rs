use std::ffi::CString;

macro_rules! test {
	($name: ident, $str: expr) => {
		#[test]
		fn $name() {
			let str = CString::new($str).expect("Cannot create str");
			let user_ret = unsafe { crate::ft_atoi(str.as_ptr()) };
			let libc_ret = unsafe { libc::atoi(str.as_ptr()) };
			assert_eq!(user_ret, libc_ret);
		}
	};
}

test!(basic, "125");
test!(whitespaces, " \t\n\r125");
test!(positive, "+125");
test!(negative, "-125");
test!(zero, "0");
test!(zero_pos, "+0");
test!(zero_neg, "-0");
test!(int_max, "2147483647");
test!(int_min, "-2147483648");
test!(text_behind, "-214feqhfhqoie");
test!(text_in_front, "-fajiof");
test!(multiple_signs, "++---++125");
test!(empty, "");
test!(combo, "       \r    \n      \t   -2147483648");
