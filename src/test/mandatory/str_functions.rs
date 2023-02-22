// I don't test crash on NULL. This is undefined behaviour.
mod strlen {
    use rusty_fork::rusty_fork_test;
    use std::ffi::CString;

    macro_rules! test {
        ($name: ident, $to_test: expr) => {
            rusty_fork_test! {
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

// Rust's libc wrapper don't have BSD/string.h to include strlcat
mod strlcat {
    use rusty_fork::rusty_fork_test;
    use std::ffi::CString;

    const fn size_of_buffer<const SIZE: usize>(table: [&str; SIZE]) -> usize {
        let mut total_size = 0_usize;
        let mut index = 0;
        loop {
            if index == SIZE {
                break total_size;
            } else {
                total_size += table[index].len();
                index += 1;
            }
        }
    }

    macro_rules! test {
        ($name: ident, $to_test: expr) => {
            test!($name, $to_test, size_of_buffer($to_test));
        };
        ($name: ident, $to_test: expr, $size: expr) => {
            rusty_fork_test!{
                #![rusty_fork(timeout_ms = 100)]

                #[test]
                fn $name() {
                    let mut current_pos = 0;
                    const MIN_BUFFER_SIZE: usize = if $size < size_of_buffer($to_test) {$size} else {size_of_buffer($to_test)};
                    let mut buffer = [0_u8; MIN_BUFFER_SIZE + 1];
                    for str in $to_test {
                        let str = CString::new(str).unwrap();
                        let ret_val = unsafe {
                            crate::ft_strlcat(buffer.as_mut_ptr() as *mut i8, str.as_ptr(), $size + 1)
                        };
                        let str_len = str.as_bytes().len();
                        assert_eq!(ret_val, current_pos + str.as_bytes().len());
                        let final_char = std::cmp::min(current_pos + str_len, MIN_BUFFER_SIZE);
                        assert_eq!(buffer[current_pos..final_char], str.as_bytes()[..final_char - current_pos]);
                        assert_eq!(buffer[final_char], 0);
                        current_pos = final_char;
                    }
                }
            }
        };
    }

    // Buffer having exact size required
    test!(basic, ["Test1", "Test2", "Super"]);
    test!(longer, ["SuperTest", "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。"]);
    test!(just_one, ["Test1"]);

    // Too small buffer
    test!(basic_small_buffer, ["Test1", "Test2", "Super"], 3);
    test!(longer_small_buffer, ["SuperTest", "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。"], 128);
    test!(just_one_small_buffer, ["Test1"], 0);

    // Bigger buffer
    test!(basic_big_buffer, ["Test1", "Test2", "Super"], 168);
    test!(longer_big_buffer, ["SuperTest", "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。"], libc::size_t::MAX - 1);
    test!(just_one_big_buffer, ["Test1"], 42);
}

mod strncmp {
    use crate::assert_same_sign;
    use std::ffi::CString;

    macro_rules! test {
        (full, $name: ident, $str1: expr, $str2: expr) => {
            test!($name, $str1, $str2, libc::size_t::MAX);
        };
        ($name: ident, $str: expr) => {
            test!($name, $str, $str, $str.len());
        };
        ($name: ident, $str1: expr, $str2: expr) => {
            test!($name, $str1, $str2, $str1.len());
        };
        ($name: ident, $str1: expr, $str2: expr, $len: expr) => {
            #[test]
            fn $name() {
                let s1 = CString::new($str1).expect("Cannot create first string");
                let s2 = CString::new($str2).expect("Cannot create second string");
                let ret_val = unsafe { crate::ft_strncmp(s1.as_ptr(), s2.as_ptr(), $len) };
                let libc_val = unsafe { libc::strncmp(s1.as_ptr(), s2.as_ptr(), $len) };
                assert_same_sign!(ret_val, libc_val);
            }
        };
    }

    // Matching
    test!(basic, "SuperTest");
    test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
    test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");
    test!(same_begin, "SuperTest!", "Soit, je comprends", 1);
    test!(no_compare, "SuperTest!", "Bof pas trop", 0);

    // Mismatch
    test!(full, basic_positive, "SuperTeste", "SuperTest");
    test!(full, basic_negative, "SuperTest", "SuperTeste");
    test!(full, longer_positive, "Lorme ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
    test!(full, longer_negative, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", "Lorme ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
    test!(full, utf8_positive, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀓麻雀🀄がしたい。このテストは本当に面白いなぁ。", "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");
    test!(full, utf8_negative, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。", "Salut! C'est un test de qualité contenant de supers UTF-8. 🀓麻雀🀄がしたい。このテストは本当に面白いなぁ。");

    // Mismatch after len
    test!(basic_after, "SuperTeste", "SuperTest", 5);
    test!(longer_after, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolores justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", 100);
    test!(utf8_after, "Salut! C'est un test de qualité contenant de supers UTF-8; 🀓麻雀🀄がしたい。このテストは本当に面白いなぁ。", "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。", 57);
}
