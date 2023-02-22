// I don't test crash on NULL. This is undefined behaviour.
mod strlen {
    use rusty_fork::rusty_fork_test;
    use std::ffi::CString;

    macro_rules! test {
        ($name: ident, $to_test: expr) => {
            rusty_fork_test!{
                #![rusty_fork(timeout_ms = 100)]

                #[test]
                fn $name() {
                    let test_str = CString::new($to_test).expect("Couldn't create string");
                    let user_ret = unsafe { crate::ft_strlen(test_str.as_ptr()) };
                    let libc_ret = unsafe { libc::strlen(test_str.as_ptr()) };
                    assert_eq!(user_ret, libc_ret);
                }
            }
        };
    }

    test!(empty, "");
    test!(basic, "SuperTest");
    test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
    test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");
}

// Rust's libc wrapper don't have BSD/string.h to include strlcpy
mod strlcpy {
    use rusty_fork::rusty_fork_test;
    use std::ffi::CString;

    macro_rules! test {
        ($name: ident, $to_test: expr) => {
            test!($name, $to_test, $to_test.len());
        };
        ($name: ident, $to_test: expr, $to_copy: expr) => {
            rusty_fork_test!{
                #![rusty_fork(timeout_ms = 100)]

                #[test]
                fn $name() {
                    let test_str = CString::new($to_test).expect("Couldn't create string");
                    let to_compare_max = std::cmp::min(test_str.as_bytes().len(), $to_copy);
                    let mut buffer = [0_u8; if $to_copy < $to_test.len() {$to_copy} else {$to_test.len()}  + 1];
                    let result = unsafe { crate::ft_strlcpy(buffer.as_mut_ptr() as *mut i8, test_str.as_ptr(), to_compare_max + 1) };
                    assert_eq!(buffer[..to_compare_max], test_str.as_bytes()[..to_compare_max]); // Check that copy occurred
                    assert_eq!(buffer[to_compare_max], 0); // Check NUL-termination
                    assert_eq!(result, $to_test.len()); // check return value
                }
            }
        };
    }

    // Full copy
    test!(empty, "");
    test!(basic, "SuperTest");
    test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
    test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");

    // Part copy
    test!(basic_part, "SuperTest", 3);
    test!(longer_part, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", 42);
    test!(utf8_part, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。", 21);

    // Given size superior to string to copy
    test!(basic_too_much, "SuperTest", 54);
    test!(longer_too_much, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", libc::size_t::MAX);
    test!(utf8_too_much, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。", 54655);

}

mod strlcat {

}
