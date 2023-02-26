//!
//! Tests for calloc
//!
//! I didn't use the libc's calloc, because it's pretty straightforward.
//! Not many things to test actually. I planned to check overflow, but figured I
//! can't really test it. I thought about checking errno, but libc sometimes set
//! it while it should have checked overflow... So guess I can't really test it.
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
