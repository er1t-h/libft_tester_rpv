use crate::{generate, libft, test::test, RANDOM_REPEAT_NUMBER};
use std::ffi::CString;

test!(
    #![test "empty string" => ""]
    ft_strdup(s: &str) {
        let str = CString::new(s).expect("DPS: couldn't create string");
        let user_string = unsafe {
            libft::ft_strdup(str.as_ptr())
        };
        let Some(user_string) = user_string else {
            panic!("returned NULL");
        };
        assert_eq!(s, user_string.as_utf8_lossy(), "wrong function output");
    }
);

crate::fork_test! {
    #[test]
    fn random_test_with_alphanumeric_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            // Generates between 2 and 500 words that will be joined by random string
            // with len between 0 and 10
            let s = generate::alnum_string();

            test(&s);
        }
    }

    #[test]
    fn random_test_with_utf8_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            // Generates between 10 and 2000 utf8 characters
            let s: String = generate::utf8_string();

            test(&s);
        }
    }
}
