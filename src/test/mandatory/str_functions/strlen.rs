use crate::{generate, libft, test::test, RANDOM_REPEAT_NUMBER};
use std::ffi::CString;

test!(
    #![test "empty string" => ""]
    ft_strlen(s: &str) {
        let test_str = CString::new(s).expect("Couldn't create string");

        // Exactly the same as libc's strlen
        let libc_ret = test_str.as_bytes().len();
        let user_ret = unsafe { libft::ft_strlen(test_str.as_ptr()) };

        assert_eq!(user_ret, libc_ret, "wrong output");
    }
);

crate::fork_test! {
    #[test]
    fn random_test_with_alphanumeric_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(&generate::alnum_string());
        }
    }

    #[test]
    fn random_test_with_utf8_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(&generate::utf8_string());
        }
    }
}
