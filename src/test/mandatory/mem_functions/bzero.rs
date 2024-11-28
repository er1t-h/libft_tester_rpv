//!
//! Tests for bzero
//!
//! I used libc::explicit_bzero because the libc crate doesn't provide a bzero.
//! It works exactly the same way. The only difference is that it guarantees the
//! execution of the function, even if optimizer would judge it pointless.
//!
//! Not testing NULL because it's undefined, so not crashing would be as
//! justifiable as crashing
//!
use crate::{
    libft,
    test::{test, Unprintable},
};
use fake::Fake;
use pretty_assertions::assert_eq;

test!(
    ft_bzero(buffer_user: Unprintable<&mut [u8]>, to_replace: usize) {
        let buffer_user = buffer_user.unwrap();
        let mut buffer_libc = buffer_user.to_vec();

        unsafe { libft::ft_bzero(buffer_user.as_mut_ptr().cast(), to_replace); };
        unsafe { libc::explicit_bzero(buffer_libc.as_mut_ptr().cast(), to_replace); };

        assert_eq!(buffer_user, buffer_libc, "your buffer and the libc buffer don't match");
    }
);

crate::fork_test! {
    #[test]
    fn random() {
        let mut buffer = fake::vec![u8; 1..10000];
        let buffer_len = buffer.len();

        test(Unprintable(Some(buffer.as_mut_slice())), (0..buffer_len).fake());
    }
}
