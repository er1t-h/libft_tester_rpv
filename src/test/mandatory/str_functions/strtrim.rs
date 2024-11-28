use crate::test::test;
use crate::{generate, libft, RANDOM_REPEAT_NUMBER};
use fake::Fake;
use pretty_assertions::assert_str_eq;
use std::ffi::CString;

test!(
    #![test "empty string" => "", "0123456789"]
    #![test "empty set" => "1HelloWorld1", ""]
    ft_strtrim(s: &str, set: &str) {
        let c_s = CString::new(s).expect("DPS: couldn't create string");
        let c_set = CString::new(set).expect("DPS: couldn't create string");

        let user_ret = unsafe {
            libft::ft_strtrim(c_s.as_ptr(), c_set.as_ptr())
        };
        let Some(user_string) = user_ret else {
            panic!("returned NULL");
        };

        let trim_begin = s.as_bytes().iter().position(|x| !set.as_bytes().contains(x));
        let trim_end = s.as_bytes().iter().rposition(|x| !set.as_bytes().contains(x));

        let expected_slice = match trim_begin.zip(trim_end) {
            Some((begin, end)) => &s.as_bytes()[begin..=end],
            None => {
                assert!(user_string.is_empty(), "the string should have been empty");
                return;
            }
        };

        assert_str_eq!(user_string.as_utf8_lossy(), String::from_utf8_lossy(expected_slice), "wrong function output")
    }
);

crate::fork_test! {
    #[test]
    fn first_null() {
        let set = CString::from(c"Super");
        let user_ret = unsafe {
            libft::ft_strtrim(std::ptr::null(), set.as_ptr())
        };
        let Some(user_string) = user_ret else {
            eprintln!("User choosed to handle by returning NULL");
            return;
        };
        if user_string.is_empty() {
            eprintln!("Creating an empty string");
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
        let str = CString::from(c"Super");
        let user_ret = unsafe {
            libft::ft_strtrim(str.as_ptr(), std::ptr::null())
        };
        eprintln!("after");
        let Some(user_string) = user_ret else {
            eprintln!("User choosed to handle by returning NULL");
            return;
        };
        assert_str_eq!(user_string.as_utf8_lossy(), String::from_utf8_lossy(str.as_bytes()), "the string should have been copied");
    }

    #[test]
    fn both_null() {
        let user_ret = unsafe {
            libft::ft_strtrim(std::ptr::null(), std::ptr::null())
        };
        let Some(user_string) = user_ret else {
            eprintln!("User choosed to handle by returning NULL");
            return;
        };
        if user_string.is_empty() {
            eprintln!("Creating an empty string");
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
    fn random_test_with_alphanumeric_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(&generate::alnum_string(), &(1..10).fake::<String>());
        }
    }

    #[test]
    fn random_test_with_utf8_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(&generate::utf8_string(), &(1..10).fake::<String>());
        }
    }
}
