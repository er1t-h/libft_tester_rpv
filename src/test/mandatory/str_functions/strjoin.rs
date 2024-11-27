use crate::{generate, libft, test::test, RANDOM_REPEAT_NUMBER};
use pretty_assertions::assert_str_eq;
use std::ffi::CString;

test!(
    #![test "first string empty" => "", "second only"]
    #![test "second string empty" => "first_only", ""]
    #![test "both string empty" => "", ""]
    ft_strjoin(s1: &str, s2: &str) {
        let c_s1 = CString::new(s1).unwrap();
        let c_s2 = CString::new(s2).unwrap();
        let user_string = unsafe {
            libft::ft_strjoin(c_s1.as_ptr(), c_s2.as_ptr())
        };

        let Some(user_string) = user_string else {
            panic!("returned NULL");
        };

        let expected = s1.to_string() + s2;
        let user_string = user_string.as_utf8_lossy();

        assert_str_eq!(user_string, expected, "wrong function output");
    }
);

crate::fork_test! {
    #[test]
    fn first_null() {
        let s2 = CString::new("anything").unwrap();
        let user_string = unsafe {
            libft::ft_strjoin(std::ptr::null(), s2.as_ptr())
        };
        let Some(user_string) = user_string else {
            eprintln!("User choosed to handle by returning NULL");
            return;
        };
        if s2.as_bytes() == user_string.as_c_str().to_bytes() {
            eprintln!("User choosed to handle by allocating only s2");
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
    fn second_null() {
        let s1 = CString::new("anything").unwrap();
        let user_string = unsafe {
            libft::ft_strjoin(s1.as_ptr(), std::ptr::null())
        };
        let Some(user_string) = user_string else {
            eprintln!("User choosed to handle by returning NULL");
            return;
        };
        if s1.as_bytes() == user_string.as_c_str().to_bytes() {
            eprintln!("User choosed to handle by allocating only s1");
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
        let user_string = unsafe {
            libft::ft_strjoin(std::ptr::null(), std::ptr::null())
        };
        let Some(user_string) = user_string else {
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
        panic!("Handled passing NULL to strjoin in a strange way");
    }

    #[test]
    fn random_test_with_alphanumeric_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            // Generates between 2 and 500 words that will be joined by random string
            // with len between 0 and 10
            let s1 = generate::alnum_string();
            let s2 = generate::alnum_string();

            test(&s1, &s2);
        }
    }

    #[test]
    fn random_test_with_utf8_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            // Generates between 10 and 2000 utf8 characters
            let s1: String = generate::utf8_string();
            let s2: String = generate::utf8_string();

            test(&s1, &s2);
        }
    }
}
