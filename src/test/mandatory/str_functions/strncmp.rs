use crate::{assert_same_sign, generate, libft, test::test, RANDOM_REPEAT_NUMBER};
use fake::Fake;
use std::ffi::CString;

test!(
    #![test "compare of size 0" => "SuperTest!", "Bof pas trop", 0]
    ft_strncmp(s1: &str, s2: &str, len: usize) {
        let c_s1 = CString::new(s1).expect("Cannot create first string");
        let c_s2 = CString::new(s2).expect("Cannot create second string");
        let ret_val = unsafe {
            libft::ft_strncmp(c_s1.as_ptr(), c_s2.as_ptr(), len)
        };
        let libc_val = unsafe {
            libc::strncmp(c_s1.as_ptr(), c_s2.as_ptr(), len)
        };
        assert_same_sign!(ret_val, libc_val, "wrong output\nlibc returned {libc_val}\nyou returned {ret_val}");
    }
);

crate::fork_test! {
    #[test]
    fn random_test_with_alphanumeric_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER * 9 / 10 {
            let s1 = generate::alnum_string();
            let s2 = generate::alnum_string();

            test(&s1, &s2, (0..8000).fake());
        }
        // since it's complicated for two random strings to match, these
        // tests will only test the same string twice
        for _ in 0..*RANDOM_REPEAT_NUMBER / 10 {
            let s = generate::alnum_string();

            test(&s, &s, (0..8000).fake());
        }
    }

    #[test]
    fn random_test_with_utf8_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER * 9 / 10 {
            let s1 = generate::utf8_string();
            let s2 = generate::utf8_string();

            test(&s1, &s2, (0..s1.len()).fake());
        }
        // since it's even more complicated for two random utf strings to match,
        // these tests will only test the same string twice
        for _ in 0..*RANDOM_REPEAT_NUMBER / 10 {
            let s = generate::utf8_string();

            test(&s, &s, (0..8000).fake());
        }
    }
}
