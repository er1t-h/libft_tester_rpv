use crate::{generate, libft, test::test, RANDOM_REPEAT_NUMBER};
use fake::Fake;
use pretty_assertions::assert_eq;
use std::ffi::CString;

test!(
    #![test "way_too_big_buffer" => "Hello, World!", 2, (isize::MAX - 10) as usize]
    ft_substr(s: &str, start: usize, len: usize) {
        let end_index = s.len().min(start.saturating_add(len));
        let expected = String::from_utf8_lossy(&s.as_bytes()[start..end_index]);

        let base_str = CString::new(s).expect("DPS: couldn't create string");
        let result = unsafe {
            libft::ft_substr(base_str.as_ptr(), start as u32, len)
        };

        let Some(user_return) = result else {
            panic!("returned NULL");
        };
        assert_eq!(
            user_return.as_utf8_lossy(), expected,
            "wrong function output"
        );
    }
);

crate::fork_test! {

    #[test]
    fn start_after_end_of_string() {
        let base_str = CString::new("SuperTest").unwrap();
        let result = unsafe {
            libft::ft_substr(base_str.as_ptr(), 1000, 6)
        };
        let Some(user_return) = result else {
            eprintln!("User choose to handle passing a start higher than len \
                            by returning NULL.");
            return;
        };
        assert!(user_return.is_empty(), "the returned string should be empty");
    }


    #[test]
    fn null() {
        let result = unsafe {
            libft::ft_substr(std::ptr::null(), 1000, 6)
        };
        assert!(result.is_none(), "With null as input, you should return NULL");
    }

    #[test]
    fn random_test_with_alphanumeric_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            let str = generate::alnum_string();
            test(&str, (2..str.len()).fake(), (0..3000).fake());
        }
    }


    #[test]
    fn random_test_with_utf8_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            let str = generate::utf8_string();
            test(&str, (2..str.as_bytes().len()).fake(), (0..5000).fake());
        }
    }
}
