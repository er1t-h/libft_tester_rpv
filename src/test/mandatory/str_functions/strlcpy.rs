use fake::Fake;

use crate::{generate, libft, test::test, RANDOM_REPEAT_NUMBER};
use pretty_assertions::{assert_eq, assert_str_eq};
use std::ffi::CString;

// Rust's libc wrapper don't have BSD/string.h to include strlcpy.
// So I'm doing the tests without it

test!(
    #![test "empty string" => "", 15]
    #![test "no buffer" => "Hello, World!", 0]
    ft_strlcpy(s: &str, nb_to_copy: usize) {
        eprintln!("Testing call ft_strlcpy(\n\t_,\n\t{s},\n\t{nb_to_copy}\n)");
        let test_str = CString::new(s).expect("Couldn't create string");
        let to_compare_max = std::cmp::min(test_str.as_bytes().len(), nb_to_copy);
        let mut buffer = vec![0_u8; nb_to_copy.min(s.as_bytes().len()) + 1];

        let result = unsafe { libft::ft_strlcpy(buffer.as_mut_ptr() as *mut i8, test_str.as_ptr(), to_compare_max + 1) };

        assert_eq!(result, s.len(), "wrong return value"); // check return value

        let user_return = String::from_utf8_lossy(&buffer[..to_compare_max]);
        let expected = String::from_utf8_lossy(&test_str.as_bytes()[..to_compare_max]);

        assert_str_eq!(user_return, expected, "wrong buffer content"); // Check that copy occurred
        assert_eq!(buffer[to_compare_max], 0, "buffer is not nul-terminated"); // Check NUL-termination
    }
);

crate::fork_test! {
    #[test]
    fn random_test_with_alphanumeric_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(&generate::alnum_string(), (2..3000).fake());
        }
    }


    #[test]
    fn random_test_with_utf8_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(&generate::utf8_string(), (10..2500).fake());
        }
    }
}
