use crate::libft;
use pretty_assertions::assert_eq;
use std::ffi::CString;

macro_rules! test {
    ($name: ident, $str: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let s = $str;
                let str = CString::new(s).expect("DPS: Cannot create str");
                let user_ret = unsafe { libft::ft_atoi(str.as_ptr()) };
                let libc_ret = unsafe { libc::atoi(str.as_ptr()) };
                assert_eq!(user_ret, libc_ret, "your output and libc's output differ for string `{}`", s);
            }
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
test!(empty, "");
test!(int_max, "2147483647");
test!(int_min, "-2147483648");
test!(text_behind, "-214feqhfhqoie");
test!(text_in_front, "-fajiof");
test!(multiple_signs, "++---++125");
test!(combo, "       \r    \n      \t   -2147483648");
