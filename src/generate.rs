use std::ops::Range;

use fake::{faker::lorem::ja_jp::Words, Fake};

// Generates between 10 and 2000 utf8 characters
pub fn utf8_string() -> String {
    utf8_string_in_range(10..2000)
}

pub fn utf8_string_in_range(r: Range<usize>) -> String {
    fake::vec![char; r]
        .into_iter()
        .filter(|&x| x != '\0')
        .collect()
}

// Generates between 2 and 500 words that will be joined by random string
// with len between 0 and 10
pub fn alnum_string() -> String {
    alnum_string_in_range(2..500, 0..10)
}

pub fn alnum_string_in_range(
    number_of_words: Range<usize>,
    separator_size: Range<usize>,
) -> String {
    Words(number_of_words)
        .fake::<Vec<String>>()
        .join(&separator_size.fake::<String>())
}
