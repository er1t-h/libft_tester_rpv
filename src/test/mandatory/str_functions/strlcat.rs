use fake::{faker::lorem::ja_jp::Words, Fake};

use crate::{
    libft,
    test::{test, DisplayableStringSlice},
    RANDOM_REPEAT_NUMBER,
};
use pretty_assertions::assert_eq;
use std::ffi::CString;

// Rust's libc wrapper don't have BSD/string.h to include strlcat

fn size_of_buffer(table: &[&str]) -> usize {
    table.iter().map(|x| x.len()).sum()
}

const LONGER_BUFFERS: &[&str] = &["SuperTest", "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。"];

test!(
    #![test "basic small buffer" => DisplayableStringSlice(&["Test1", "Test2", "Super"]), 3]
    #![test "longer input with small buffer" => DisplayableStringSlice(LONGER_BUFFERS), 128]
    #![test "empty buffer" => DisplayableStringSlice(&["Test1"]), 0]
    #![test "bigger buffer with small input" => DisplayableStringSlice(&["Test1", "Test2", "Super"]), 168]
    #![test "passing size_t max as buffer size" => DisplayableStringSlice(LONGER_BUFFERS), libc::size_t::MAX - 1]
    #![test "just one string with big buffer" => DisplayableStringSlice(&["Test1"]), 42]
    ft_strlcat(input: DisplayableStringSlice<&str>, final_buffer_size: usize) {
        let input = input.0;
        let mut current_pos = 0;
        let size_of_input = size_of_buffer(input);
        let min_buffer_size: usize = final_buffer_size.min(size_of_input);
        let mut buffer = vec![0_u8; min_buffer_size + 1];
        for &str in input {
            let str = CString::new(str).unwrap();
            let ret_val = unsafe {
                libft::ft_strlcat(buffer.as_mut_ptr() as *mut i8, str.as_ptr(), final_buffer_size + 1)
            };
            let str_len = str.as_bytes().len();
            assert_eq!(ret_val, current_pos + str.as_bytes().len());
            let final_char = std::cmp::min(current_pos + str_len, min_buffer_size);
            assert_eq!(buffer[current_pos..final_char], str.as_bytes()[..final_char - current_pos]);
            assert_eq!(buffer[final_char], 0);
            current_pos = final_char;
        }
    }
);

crate::fork_test! {
    #[test]
    fn with_3_small_buffers() {
        let input = &["Test1", "Test2", "Super"];
        test(DisplayableStringSlice(input), size_of_buffer(input));
    }

    #[test]
    fn with_3_bigger_buffers() {
        let input = LONGER_BUFFERS;
        test(DisplayableStringSlice(input), size_of_buffer(input));
    }

    #[test]
    fn with_just_one_buffer() {
        let input = &["Test1"];
        test(DisplayableStringSlice(input), size_of_buffer(input));
    }

    #[test]
    fn random_test_with_alphanumeric_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            // Generates between 2 and 500 words that will be joined by random string
            // with len between 0 and 10
            let s: Vec<String> = Words(2..500).fake();
            let input: Vec<_> = s.iter().map(|x| x.as_str()).collect();

            test(DisplayableStringSlice(&input), (2..3000).fake());
        }
    }

    #[test]
    fn random_test_with_utf8_characters() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            // Generates between 10 and 2000 utf8 characters
            let s: Vec<String> = {
                let len = (3..50).fake();
                let mut v = Vec::with_capacity(len);
                for _ in 0..len {
                    let c = fake::vec![char; 2..20];
                    v.push(c.into_iter().filter(|&x| x != '\0').collect());
                }
                v
            };
            let input: Vec<_> = s.iter().map(|x| x.as_str()).collect();

            test(DisplayableStringSlice(&input), (10..2500).fake());
        }
    }
}
