use libc::c_void;

macro_rules! test {
	($name: ident, $nb: expr) => {
		crate::fork_test!{
			#[test]
			fn $name() {
				let mut list = std::ptr::null_mut();
				for i in 0_usize..$nb {
					unsafe {
						crate::ft_lstadd_front(&mut list, crate::ft_lstnew(
							i as *mut c_void
						))
					}
				}
				for i in (0_usize..$nb).rev() {
					let current = list;
					let content = unsafe { (*current).content } as usize;
					assert_eq!(content, i, "Element mismatch. Either an addback failed, or the order got mixed up.");
					list = (unsafe { *list }).next;
					unsafe { libc::free(current.cast()) };
				}
			}
		}
	};
}

test!(just_one, 1);
test!(basic, 3);
test!(many_items, 100);
