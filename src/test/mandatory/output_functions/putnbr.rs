use fake::{Fake, Faker};
use libc::c_int;

use crate::{libft, test::test, RANDOM_REPEAT_NUMBER};
use std::{fs::File, io::Read, os::fd::FromRawFd};

test!(
    #![test "zero" => 0]
    #![test "int min" => c_int::MIN]
    #![test "int max" => c_int::MAX]
    ft_putnbr_fd(nb: c_int) {
        unsafe {
            let [read, write] = super::pipe();
            let _write = File::from_raw_fd(write);
            let mut read = File::from_raw_fd(read);
            libft::ft_putnbr_fd(nb, write);
            std::mem::drop(_write);
            let mut buffer = Vec::new();
            read.read_to_end(&mut buffer).expect("DPS: couldn't read");
            assert_eq!(String::from_utf8_lossy(&buffer), nb.to_string(), "didn't print the right thing");
        }
    }
);

crate::fork_test! {
    #[test]
    fn random_tests() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test(Faker.fake());
        }
    }
}
