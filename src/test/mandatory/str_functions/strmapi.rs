use libc::{c_char, c_uint};

use crate::{
    generate, libft,
    test::{test, Unprintable},
    utils, RANDOM_REPEAT_NUMBER,
};
use std::ffi::CString;

test!(
    #![test "empty string" => "", Unprintable(Some(utils::rotone))]
    ft_strmapi(s: &str, f: Unprintable<unsafe extern "C" fn(c_uint, c_char) -> c_char>) {
        let f = f.unwrap();
        let cchars = CString::new(s).unwrap();
        let mapi_return = unsafe { libft::ft_strmapi(cchars.as_ptr(), Some(f)) };

        let Some(user_string) = mapi_return else {
            panic!("returned NULL");
        };

        // `.enumerate().map()` is literally the Rust equivalent of `mapi`.
        // And we apply exactly the same function (f) in our rust code and in C
        let expected: Vec<_> = cchars.as_bytes().iter().enumerate().map(|(index, value)| unsafe { f(index as u32, *value as i8) as u8 }).collect();
        let expected = String::from_utf8_lossy(&expected);

        assert_eq!(user_string.as_utf8_lossy(), expected, "wrong output");
    }
);

crate::fork_test! {
    #[test]
    fn str_as_null() {
        unsafe { libft::ft_strmapi(std::ptr::null(), Some(utils::rotx)) };
    }

    #[test]
    fn fn_as_null() {
        let content = c"Bonjour";

        let expected = CString::from(content);
        let ret = unsafe { libft::ft_strmapi(expected.as_ptr(), None) };
        let Some(user_string) = ret else {
            eprintln!("User chose to return NULL");
            return;
        };
        if content == user_string.as_c_str() {
            eprintln!("User chose to return a copy of str");
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
        unsafe { libft::ft_strmapi(std::ptr::null_mut(), None) };
    }

    #[test]
    fn random_test_with_alphanumeric_characters_rotone() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(&generate::alnum_string(), Unprintable(Some(utils::rotone)));
        }
    }

    #[test]
    fn random_test_with_utf8_characters_rotone() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(&generate::utf8_string(), Unprintable(Some(utils::rotone)));
        }
    }

    #[test]
    fn random_test_with_alphanumeric_characters_rotx() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(&generate::alnum_string(), Unprintable(Some(utils::rotx)));
        }
    }

    #[test]
    fn random_test_with_utf8_characters_rotx() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(&generate::utf8_string(), Unprintable(Some(utils::rotx)));
        }
    }

    #[test]
    fn random_test_with_alphanumeric_characters_to_num() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(&generate::alnum_string(), Unprintable(Some(utils::to_num)));
        }
    }

    #[test]
    fn random_test_with_utf8_characters_to_num() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(&generate::utf8_string(), Unprintable(Some(utils::to_num)));
        }
    }
}
