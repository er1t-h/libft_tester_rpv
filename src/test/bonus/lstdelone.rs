use libc::c_void;

macro_rules! test {
    ($name: ident, $nb: expr) => {
        crate::fork_test! {
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
                let size = unsafe { crate::ft_lstsize(list) };
                assert_eq!(size, $nb);
                for _i in 0_usize..$nb {
                    let mut tmp = list;
                    list = (unsafe { *list }).next;
                    unsafe { crate::ft_lstdelone(&mut tmp, Some(crate::nofree)) };
                }
            }
        }
    };
}

test!(just_one, 1);
test!(basic, 3);
test!(many_items, 100);
test!(empty, 0);
