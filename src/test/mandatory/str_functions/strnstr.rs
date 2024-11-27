use fake::Fake;
use pretty_assertions::assert_str_eq;

use crate::{generate, libft, test::test, RANDOM_REPEAT_NUMBER};
use std::ffi::CString;

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        // if needle is empty string, then haystack is returned
        Some(0)
    } else {
        haystack.windows(needle.len()).position(|hay| hay == needle)
    }
}

test!(
    #![test "match but size 0" => "match","m",0]
    #![test "empty needle" => "match","",5]
    #![test "no match n max" => "Un test intéressant","teste",libc::size_t::MAX]
    #![test "match trap" => "bonbonbons", "bonbons", libc::size_t::MAX]
    #![test "stopping during match" => "Super test", "test", 8]
    ft_strnstr(haystack: &str, needle: &str, size: usize) {
        let c_haystack = CString::new(haystack).expect("DPS: couldn't create string");
        let c_needle = CString::new(needle).expect("DPS: couldn't create string");

        let user_ret = unsafe { libft::ft_strnstr(c_haystack.as_ptr(), c_needle.as_ptr(), size) };
        match (find(&haystack.as_bytes()[..size.min(haystack.len())], needle.as_bytes()), user_ret) {
            (None, None) => (),
            (None, Some(_)) => panic!("needle isn't in the first characters of haystack, yet you found it"),
            (Some(_), None) => panic!("needle was in the first characters of haystack, but you didn't find it"),
            (Some(expected), Some(user_ret)) => {
                let expected_string = String::from_utf8_lossy(&haystack.as_bytes()[expected..]);
                let user_string = String::from_utf8_lossy(user_ret.to_bytes());

                assert_str_eq!(user_string, expected_string, "wrong function output");
            }
        }
    }
);

crate::fork_test! {
    #[test]
    fn random_test_with_alphanumeric_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            let s = generate::alnum_string();

            test(&s, &(2..8).fake::<String>(), (0..s.len()).fake());
        }
    }

    #[test]
    fn random_test_with_utf8_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            let s: String = generate::utf8_string();

            test(&s, &(1..4).fake::<String>(), (0..s.len()).fake());
        }
    }
}
