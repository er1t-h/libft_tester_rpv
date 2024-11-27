use crate::{
    libft,
    test::{test, DisplayableSlice},
    RANDOM_REPEAT_NUMBER,
};
use fake::Fake;
use libc::c_void;
use pretty_assertions::assert_eq;

test!(
    ft_memcpy(src: DisplayableSlice<u8>, size: usize) {
        let src = src.0;
        let mut user_buffer = vec![0; src.len()];
        let mut libc_buffer = vec![0; src.len()];
        let Some(user_ret) = (unsafe { libft::ft_memcpy(user_buffer.as_mut_ptr() as *mut c_void, src.as_ptr() as *mut c_void, size) }) else {
            panic!("returned NULL");
        };
        assert_eq!(user_ret, user_buffer.as_mut_ptr().cast(), "return value and dest do not match");
        unsafe { libc::memcpy(libc_buffer.as_mut_ptr() as *mut c_void, src.as_ptr() as *mut c_void, size) };
        assert_eq!(user_buffer, libc_buffer, "the two buffers differ");
    }
);

crate::fork_test! {
    #[test]
    fn random() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            let source = fake::vec![u8; 1..5000];
            test(DisplayableSlice(&source), (0..source.len()).fake());
        }
    }
}
