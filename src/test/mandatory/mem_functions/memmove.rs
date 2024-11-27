use crate::{
    libft,
    test::{test, DisplayableSlice},
    RANDOM_REPEAT_NUMBER,
};
use fake::Fake;
use pretty_assertions::assert_eq;

test!(
    ft_memmove(buffer_user: DisplayableSlice<u8>, src: usize, dest: usize, to_copy: usize) {
        let mut buffer_user = buffer_user.0.to_vec();
        let mut buffer_libc = buffer_user.clone();

        unsafe {
            libc::memmove(buffer_libc.as_mut_ptr().add(dest).cast(), buffer_libc.as_ptr().add(src).cast(), to_copy);
            let ret = libft::ft_memmove(buffer_user.as_mut_ptr().add(dest).cast(), buffer_user.as_ptr().add(src).cast(), to_copy);
            assert_eq!(Some(buffer_user.as_ptr().add(dest).cast_mut().cast()), ret, "did not return a pointer to dest");
        }

        assert_eq!(buffer_user, buffer_libc, "the two buffers mismatch");
    }
);

crate::fork_test! {
    #[test]
    fn random() {
        eprintln!("how to read this test:");
        eprintln!("ft_memmove(`buffer`, `src`, `dest`, `to_copy`);");
        eprintln!("given the input buffer, it tests ft_memmove(buffer + dest, buffer + src, to_copy)");

        for _ in 0..*RANDOM_REPEAT_NUMBER {
            let buffer = fake::vec![u8; 1..8000];
            let len = buffer.len();
            let src: usize = (0..len).fake();
            let dest = (0..len).fake();
            test(DisplayableSlice(&buffer), src, dest, (0..(len - dest.max(src))).fake());
        }
    }
}
