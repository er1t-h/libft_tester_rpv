use crate::{fork_test, libft, test::test};
use std::{fs::File, io::Read, os::fd::FromRawFd};

test!(
    ft_putchar_fd(c: char) {
        unsafe {
            let [read, write] = super::pipe();
            let _write = File::from_raw_fd(write);
            let mut read = File::from_raw_fd(read);
            libft::ft_putchar_fd(c as i8, write);
            std::mem::drop(_write);
            let mut buffer = Vec::new();
            read.read_to_end(&mut buffer).expect("DPS: couldn't read");
            assert_eq!(buffer, &[c as u8], "didn't print the right thing");
        }
    }
);

fork_test! {
    #[test]
    fn all_chars() {
        for c in 0..=255_u8 {
            test(c as char)
        }
    }
}
