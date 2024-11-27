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
                unsafe { crate::ft_lstclear(&mut list, Some(crate::nofree)) };
                assert!(list.is_null(), "List pointer is not changed to NULL.");
            }
        }
    };
}

test!(just_one, 1);
test!(basic, 3);
test!(many_items, 100);

crate::fork_test! {
    #[test]
    fn del_as_null() {
        let mut list = std::ptr::null_mut();
        for i in 0_usize..20 {
            unsafe {
                crate::ft_lstadd_front(&mut list, crate::ft_lstnew(
                    i as *mut c_void
                ))
            }
        }
        unsafe { crate::ft_lstclear(&mut list, None) };
        assert!(list.is_null(), "List pointer is not changed to NULL.");
    }
}
