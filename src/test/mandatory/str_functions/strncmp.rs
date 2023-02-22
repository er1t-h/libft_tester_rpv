use std::ffi::CString;
use crate::assert_same_sign;

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
			let ret_val = unsafe {
				crate::ft_strncmp(s1.as_ptr(), s2.as_ptr(), $len)
			};
			let libc_val = unsafe {
				libc::strncmp(s1.as_ptr(), s2.as_ptr(), $len)
			};
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
test!(utf8_, "Salut! C'est un test de qualité contenant de supers UTF-8; 🀓麻雀🀄がしたい。このテストは本当に面白いなぁ。", "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。", 57);
