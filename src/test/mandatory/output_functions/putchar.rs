use std::{fs::File, os::unix::prelude::AsRawFd};

macro_rules! test {
    ($name: ident, $to_write: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let filename = format!(".tests_putchar/{}.txt", line!());
                let file = File::create(&filename).unwrap();
                let fd = file.as_raw_fd();
                for c in $to_write.as_bytes() {
                    unsafe { crate::ft_putchar_fd(*c as i8, fd) }
                }
                let content = std::fs::read_to_string(filename).unwrap();
                assert_eq!(content, $to_write);
            }
        }
    };
}

test!(basic, "Super !");
test!(
    longer,
    "En vrai faire un call a write pour chaque caractere c'est pas ouf"
);
test!(utf8, "Salut! C'est un test de qualitรฉ contenant de supers UTF-8. ๐้บป้๐ใใใใใใใฎใในใใฏๆฌๅฝใซ้ข็ฝใใชใใ");
