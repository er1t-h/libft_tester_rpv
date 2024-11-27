use libc::c_void;

macro_rules! test {
    ($name: ident, $nb: expr) => {
        crate::fork_test!{
            #[test]
            fn $name() {
                let mut list = std::ptr::null_mut();
                for i in 0_usize..$nb {
                    unsafe {
                        crate::ft_lstadd_back(&mut list, crate::ft_lstnew(
                            i as *mut c_void
                        ))
                    }
                }
                for i in 0_usize..$nb {
                    let tmp = list;
                    list = (unsafe { *list }).next;
                    let content = unsafe { (*tmp).content } as usize;
                    assert_eq!(content, i, "Element mismatch. Either an addfront failed, or the order got mixed up.");
                    unsafe { libc::free(tmp.cast()) };
                }
            }
        }
    };
}

test!(just_one, 1);
test!(basic, 3);
test!(many_items, 100);
