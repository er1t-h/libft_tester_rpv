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
use crate::{libft, test::test, RANDOM_REPEAT_NUMBER};
use fake::Fake;

test!(
    ft_calloc(nmemb: usize, size: usize) {
        let user_ptr = unsafe { libft::ft_calloc(nmemb, size) };
        let Some(user_ptr) = user_ptr else {
            panic!("returned NULL");
        };
        let user_slice = unsafe { std::slice::from_raw_parts(user_ptr.as_ptr().cast::<u8>(), nmemb * size) };
        if let Some(position) = user_slice.iter().position(|&x| x != 0) {
            panic!("found something other than 0 at {position}")
        }
    }
);

crate::fork_test! {
    #[test]
    fn left_zero() {
        unsafe { libft::ft_calloc(0, 10) };
        // either it's null, and we don't do anything
        // or it's some, and it will automatically be freed
    }
    #[test]
    fn right_zero() {
        unsafe { libft::ft_calloc(10, 0) };
    }
    #[test]
    fn both_zero() {
        unsafe { libft::ft_calloc(0, 0) };
    }
    #[test]
    fn overflow() {
        assert!(unsafe { libft::ft_calloc(usize::MAX - 11, usize::MAX - 10) }.is_none(), "should return null on overflow");
    }
    #[test]
    fn random() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            test((1..50).fake(), (1..50).fake());
        }
    }
}
