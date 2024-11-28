use crate::{generate, RANDOM_REPEAT_NUMBER};
use crate::{libft, test::test};
use pretty_assertions::assert_eq;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::os::fd::FromRawFd;

test!(
    ft_putendl_fd(s: &str) {
        let string = CString::new(s).expect("DPS: couldn't create string");
        unsafe {
            let [read, write] = super::pipe();
            let _write = File::from_raw_fd(write);
            let mut read = File::from_raw_fd(read);
            libft::ft_putendl_fd(string.as_ptr(), write);
            std::mem::drop(_write);
            let mut buffer = Vec::new();
            read.read_to_end(&mut buffer).expect("DPS: couldn't read");
            let (last, init) = buffer.split_last().expect("nothing was printed");
            assert_eq!(init, string.as_bytes(), "didn't print the right thing");
            assert_eq!(*last as char, '\n', "didn't print newline");
        }
    }
);

crate::fork_test! {
    #[test]
    fn null() {
        unsafe {
            let [read, write] = super::pipe();
            let _write = File::from_raw_fd(write);
            let mut read = File::from_raw_fd(read);
            libft::ft_putendl_fd(std::ptr::null(), write);
            std::mem::drop(_write);
            let mut buffer = Vec::new();
            read.read_to_end(&mut buffer).expect("DPS: couldn't read");
            assert!(buffer.is_empty() || buffer == [b'\n'], "shouldn't have printed anything, or just a newline");
        }
    }

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
