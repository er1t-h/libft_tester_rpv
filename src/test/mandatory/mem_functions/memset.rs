//!
//! Tests for memset
//!
//! Not testing NULL because it's undefined, so not crashing would be as
//! justifiable as crashing
//!
use crate::{
    libft,
    test::{test, DisplayableSlice},
};
use fake::{Fake, Faker};

test!(
    ft_memset(buffer_user: DisplayableSlice<u8>, character: char, to_replace: usize) {
        let mut buffer_user = buffer_user.0.to_vec();
        let mut buffer_libc = buffer_user.to_vec();

        unsafe { libft::ft_memset(buffer_user.as_mut_ptr().cast(), character as i32, to_replace); };
        unsafe { libc::memset(buffer_libc.as_mut_ptr().cast(), character as i32, to_replace); };

        assert_eq!(buffer_user, buffer_libc, "your buffer and the libc buffer don't match");
    }
);

crate::fork_test! {
    #[test]
    fn random() {
        let buffer = fake::vec![u8; 1..10000];
        let buffer_len = buffer.len();

        test(DisplayableSlice(buffer.as_slice()), Faker.fake::<u8>() as char, (0..buffer_len).fake());
    }
}
