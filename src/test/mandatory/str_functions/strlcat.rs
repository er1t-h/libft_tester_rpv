use std::ffi::CString;

// Rust's libc wrapper don't have BSD/string.h to include strlcat

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
		crate::fork_test!{
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
