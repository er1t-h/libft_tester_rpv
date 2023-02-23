use std::{fs::File, os::unix::prelude::AsRawFd};
use std::ffi::CString;

macro_rules! test {
	($name: ident, $to_write: expr) => {
		#[test]
		fn $name() {
			let filename = format!(".tests_putendl/{}.txt", line!());
			let file = File::create(&filename).unwrap();
			let fd = file.as_raw_fd();
			let to_print = CString::new($to_write).unwrap();
			unsafe { crate::ft_putendl_fd(to_print.as_ptr(), fd) }
			let content = std::fs::read_to_string(filename).unwrap();
			assert_eq!(&content[..content.len() - 1], $to_write, "Doesn't match");
			assert_eq!(content.as_str().chars().last().unwrap(), '\n', "Didn't put the final `\\n`");
		}
	};
}
test!(basic, "Super !");
test!(longer, "En vrai faire un call a write pour chaque caractere c'est pas ouf");
test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");

#[test]
fn null() {
	let filename = format!(".tests_putendl/{}.txt", line!());
	let file = File::create(&filename).unwrap();
	let fd = file.as_raw_fd();
	unsafe { crate::ft_putendl_fd(std::ptr::null(), fd) }
}
