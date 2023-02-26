use std::ffi::CString;

macro_rules! test {
    ($name: ident, $str: expr, $to_find: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let str = CString::new("Test basique").unwrap();
                let ret_user = unsafe {
                    crate::ft_strrchr(str.as_ptr(), $to_find as i32)
                };
                let ret_libc = unsafe {
                    libc::strrchr(str.as_ptr(), $to_find as i32)
                };
                assert_eq!(ret_user, ret_libc);
            }
        }
    };
}

test!(basic, "Test basique", b'e');
test!(no_entry, "Test basique", b'r');
test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", b'v');
test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。", '🀄');
test!(longer_no_entry, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", b'z');
