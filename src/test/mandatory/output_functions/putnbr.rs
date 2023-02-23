use std::{fs::File, os::unix::prelude::AsRawFd};

macro_rules! test {
	($name: ident, $to_write: expr) => {
		crate::fork_test! {
			#[test]
			fn $name() {
				let filename = format!(".tests_putnbr/{}.txt", line!());
				let file = File::create(&filename).unwrap();
				let fd = file.as_raw_fd();
				unsafe { crate::ft_putnbr_fd($to_write, fd) }
				let content = std::fs::read_to_string(filename).unwrap();
				assert_eq!(&content[..content.len()], $to_write.to_string(), "Doesn't match");
			}
		}
	};
}

test!(basic, 11037);
test!(basic_negative, -11037);
test!(int_min, -2147483648);
test!(int_max, 2147483647);
test!(zero, 0);
