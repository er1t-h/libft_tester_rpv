use std::ffi::CString;

macro_rules! test {
	($name: ident, $haystack: expr, $needle: expr, $size: expr) => {
		crate::fork_test!{
			#![rusty_fork(timeout_ms = 100)]

			#[test]
			fn $name() {
				let haystack = CString::new($haystack).unwrap();
				let needle = CString::new($needle).unwrap();
				let user_ret = unsafe {
					crate::ft_strnstr(haystack.as_ptr(), needle.as_ptr(), $size)
				};
				match $haystack[..if $size > $haystack.len() {$haystack.len()} else {$size}].find($needle) {
					None => assert!(user_ret.is_null(), "There should have been no match."),
					Some(x) => assert_eq!(x, user_ret as usize - haystack.as_ptr() as usize),
				}
			}
		}
	};
	($name: ident, $haystack: expr, $needle: expr) => {
		test!($name, $haystack, $needle, $haystack.len());
	};
}

test!(basic, "Un test intéressant", "test");
test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", "lobortis");
test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。", "麻雀");
test!(match_trap, "bonbonbons", "bonbons");
test!(no_match, "Un test intéressant", "teste");
test!(
    no_match_n_max,
    "Un test intéressant",
    "teste",
    libc::size_t::MAX
);
test!(stop_in_match, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。", "麻雀", 70);
