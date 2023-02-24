crate::fork_test! {
    #[test]
    fn basic() {
        let mut buffer = *b"Value";
        let user_ret = unsafe {
            crate::ft_lstnew(buffer.as_mut_ptr().cast())
        };
        assert!(!user_ret.is_null(), "lstnew returned null");
        let content = unsafe {
            std::slice::from_raw_parts((*user_ret).content.cast::<u8>(), buffer.len())
        };
        assert_eq!(content, buffer);
        unsafe { libc::free(user_ret.cast()) };
    }

    #[test]
    fn null() {
        let user_ret = unsafe {
            crate::ft_lstnew(std::ptr::null_mut())
        };
        assert!(!user_ret.is_null(), "lstnew returned null");
        assert!((unsafe { *user_ret }).content.is_null());
        unsafe { libc::free(user_ret.cast()) };
    }
}
