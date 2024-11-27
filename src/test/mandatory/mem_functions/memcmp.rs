use crate::{
    assert_same_sign, libft,
    test::{test, DisplayableSlice},
    RANDOM_REPEAT_NUMBER,
};
use fake::Fake;
use libc::c_void;

test!(
    ft_memcmp(first_buffer: DisplayableSlice<u8>, second_buffer: DisplayableSlice<u8>, size: usize) {
        let first_buffer = first_buffer.0;
        let second_buffer = second_buffer.0;
        let user_ret = unsafe {
            libft::ft_memcmp(first_buffer.as_ptr() as *const c_void, second_buffer.as_ptr() as *const c_void, size)
        };
        let libc_ret = unsafe {
            libc::memcmp(first_buffer.as_ptr() as *const c_void, second_buffer.as_ptr() as *const c_void, size)
        };
        assert_same_sign!(user_ret, libc_ret, "sign mismatch");
    }
);

crate::fork_test! {
    #[test]
    fn random() {
        for _ in 0..*RANDOM_REPEAT_NUMBER * 9 / 10 {
            let size = (1..5000).fake();
            let first_buffer = fake::vec![u8; size];
            let second_buffer = fake::vec![u8; size];
            test(DisplayableSlice(&first_buffer), DisplayableSlice(&second_buffer), (0..size).fake());
        }

        for _ in 0..*RANDOM_REPEAT_NUMBER / 10 {
            let size = (1..5000).fake();
            let first_buffer = fake::vec![u8; size];
            test(DisplayableSlice(&first_buffer), DisplayableSlice(&first_buffer), (0..size).fake());
        }
    }
}
