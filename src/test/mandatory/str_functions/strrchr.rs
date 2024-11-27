use fake::{Fake, Faker};

use crate::test::test;
use crate::{generate, libft, RANDOM_REPEAT_NUMBER};
use pretty_assertions::assert_str_eq;
use std::ffi::CString;

test!(
    #![test "finding nul character" => "bonjour", '\0']
    #![test "searching empty string" => "", 'H']
    ft_strrchr(s: &str, c: char) {
        let input_string = CString::new(s).expect("DPS: couldn't create string");

        let ret_user = unsafe {
            libft::ft_strrchr(input_string.as_ptr(), c as i8 as i32)
        };
        let ret_libc = unsafe {
            libc::strrchr(input_string.as_ptr(), c as i8 as i32)
        };

        if ret_libc.is_null() {
            assert!(ret_user.is_none(), "libc returned null but not user");
        } else if let Some(string_user) = ret_user {
            unsafe {
                let skipped_libc = ret_libc.offset_from(input_string.as_ptr()).unsigned_abs();
                let skipped_user = string_user.as_ptr().offset_from(input_string.as_ptr()).unsigned_abs();
                if skipped_libc == skipped_user { return; }

                let slice_libc = std::slice::from_raw_parts(ret_libc.cast(), input_string.as_bytes().len() - skipped_libc);

                let str_libc = String::from_utf8_lossy(slice_libc);
                let str_user = String::from_utf8_lossy(string_user.to_bytes());

                assert_str_eq!(str_user, str_libc);
            }
        } else {
            panic!("user returned null, but not libc");
        }
    }
);

crate::fork_test! {
    #[test]
    fn random_test_with_alphanumeric_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            // Generates between 2 and 500 words that will be joined by random string
            // with len between 0 and 10
            let s = generate::alnum_string();

            test(&s, Faker.fake::<u8>() as char);
        }
    }

    #[test]
    fn random_test_with_utf8_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            // Generates between 10 and 2000 utf8 characters
            let s: String = generate::utf8_string();

            test(&s, Faker.fake::<u8>() as char);
        }
    }
}
