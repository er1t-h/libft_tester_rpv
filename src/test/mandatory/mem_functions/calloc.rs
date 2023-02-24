//!
//! Tests for calloc
//!
//! I didn't use the libc's calloc, because it's pretty straightforward.
//! Not many things to test actually.
//! For the zero tests, as long as you don't crash or return a wrong pointer,
//! everything should be good
//!

crate::fork_test! {
    #[test]
    fn basic() {
        let user_ret = unsafe { crate::ft_calloc(50, 10) };
        let content = unsafe { std::slice::from_raw_parts(user_ret as *mut u8, 500) };
        assert_eq!(content, &[0; 500]);
        unsafe { libc::free(user_ret) };
    }

    #[test]
    fn check_overflow() {
        let user_ret = unsafe { crate::ft_calloc(libc::size_t::MAX, 2) };
        let user_errno = unsafe { *libc::__errno_location() };
        unsafe { libc::calloc(libc::size_t::MAX, 2) };
        let libc_errno = unsafe { *libc::__errno_location() };
        assert!(user_ret.is_null());
        assert_ne!(user_errno, libc_errno, "When param1 * param2 overflows, calloc should return an error (NULL). But yours tries to malloc, thus setting ENOMEM");
    }

    #[test]
    fn left_zero() {
        let user_ret = unsafe { crate::ft_calloc(0, 10) };
        unsafe { libc::free(user_ret) };
    }
    #[test]
    fn right_zero() {
        let user_ret = unsafe { crate::ft_calloc(10, 0) };
        unsafe { libc::free(user_ret) };
    }
    #[test]
    fn both_zero() {
        let user_ret = unsafe { crate::ft_calloc(0, 0) };
        unsafe { libc::free(user_ret) };
    }
}
