use libc::{c_char, c_uint};
use pretty_assertions::assert_str_eq;

use crate::{
    generate, libft,
    test::{test, Unprintable},
    utils, RANDOM_REPEAT_NUMBER,
};
use std::ffi::CString;

test!(
    #![test "empty string with rotone" => String::new().as_mut_str(), Unprintable(Some(utils::rotone_in_place))]
    #![test "empty string with rotx" => String::new().as_mut_str(), Unprintable(Some(utils::rotx_in_place))]
    #![test "empty string with to_num" => String::new().as_mut_str(), Unprintable(Some(utils::to_num_in_place))]
    ft_striteri(s: &str, f: Unprintable<unsafe extern "C" fn(c_uint, *mut c_char)>) {
        let f = f.unwrap();
        let mut s = s.to_string();
        let cchars = CString::new(s.as_str()).expect("DPS: couldn't create string");

        unsafe {
            libft::ft_striteri(cchars.as_ptr().cast_mut(), Some(f));
            s.as_bytes_mut().iter_mut().enumerate().for_each(|(index, value)| f(index as u32, (value as *mut u8).cast()));
        }
        let user_return = String::from_utf8_lossy(cchars.as_bytes());
        assert_str_eq!(user_return, s.as_str(), "wrong output");
    }
);

crate::fork_test! {
    #[test]
    fn str_as_null() {
        unsafe { libft::ft_striteri(std::ptr::null_mut(), Some(utils::rotx_in_place)) }
    }

    #[test]
    fn fn_as_null() {
        let expected = c"Bonjour";
        let cchars = CString::from(expected);
        unsafe { libft::ft_striteri(cchars.as_ptr().cast_mut(), None) }

        let expected = String::from_utf8_lossy(expected.to_bytes());
        let user_return = String::from_utf8_lossy(cchars.as_bytes());

        assert_eq!(user_return, expected, "string was changed but function was NULL");
    }

    #[test]
    fn both_null() {
        unsafe { libft::ft_striteri(std::ptr::null_mut(), None) }
    }

    #[test]
    fn random_test_with_alphanumeric_characters_with_rotone() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(generate::alnum_string().as_mut_str(), Unprintable(Some(utils::rotone_in_place)));
        }
    }

    #[test]
    fn random_test_with_utf8_characters_with_rotone() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(generate::utf8_string().as_mut_str(), Unprintable(Some(utils::rotone_in_place)));
        }
    }

    #[test]
    fn random_test_with_alphanumeric_characters_with_rotx() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(generate::alnum_string().as_mut_str(), Unprintable(Some(utils::rotx_in_place)));
        }
    }

        #[test]
    fn random_test_with_utf8_characters_with_rotx() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(generate::utf8_string().as_mut_str(), Unprintable(Some(utils::rotx_in_place)));
        }
    }

    #[test]
    fn random_test_with_alphanumeric_characters_with_to_num() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(generate::alnum_string().as_mut_str(), Unprintable(Some(utils::to_num_in_place)));
        }
    }

        #[test]
    fn random_test_with_utf8_characters_with_to_num() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(generate::utf8_string().as_mut_str(), Unprintable(Some(utils::to_num_in_place)));
        }
    }
}
