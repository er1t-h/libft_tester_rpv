use crate::{generate, libft, test::test, RANDOM_REPEAT_NUMBER};
use fake::{Fake, Faker};
use libc::c_char;
use pretty_assertions::{assert_eq, assert_str_eq};
use std::ffi::CString;

test!(
    #![test "empty string" => "", 'a']
    #![test "delimiter is nul character" => "Hello, World!", '\0']
    ft_split(s: &str, delimiter: char) {
        let input_str = CString::new(s).expect("DPS: couldn't create string");
        let user_table = unsafe {
            libft::ft_split(input_str.as_ptr(), delimiter as c_char)
        };
        let Some(user_table) = user_table else {
            panic!("returned NULL");
        };

        let expected: Vec<&[u8]> = s.as_bytes().split(|&x| x as i8 == delimiter as i8).filter(|x| !x.is_empty()).collect();

        assert_eq!(user_table.len(), expected.len(), "wrong table len");

        for (i, part) in expected.into_iter().filter(|x| !x.is_empty()).enumerate() {
            let cstr = &user_table[i];
            let user_ret = String::from_utf8_lossy(cstr.to_bytes());

            let expected_string = String::from_utf8_lossy(part);

            assert_str_eq!(user_ret, expected_string, "wrong output for part {i}");
        }
    }
);

crate::fork_test! {
    #[test]
    fn null() {
        let user_table = unsafe {
            libft::ft_split(std::ptr::null(), 'v' as i8)
        };
        let Some(user_table) = user_table else {
            eprintln!("User chose to return NULL when NULL is given to split");
            return;
        };
        // If you go through here, you handled passing NULL in a way I didn't anticipate.
        // If you can explain it clearly, you may ignore this failing test.
        // However, if you let your code crash with this test
        // because you didn't handle having NULL as an argument, you should get
        // a Crash flag.
        assert!(user_table.is_empty(), "Handled passing NULL to split in a strange way");
    }


    #[test]
    fn random_test_with_alphanumeric_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            let s = generate::alnum_string();

            test(&s, Faker.fake::<u8>() as char);
        }
    }

    #[test]
    fn random_test_with_utf8_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            let s: String = generate::utf8_string();

            test(&s, Faker.fake::<u8>() as char);
        }
    }
}
