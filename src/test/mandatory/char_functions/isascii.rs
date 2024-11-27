use libc::c_int;

use crate::{assert_nzero, fork_test, libft, test::test};

test!(
    ft_isascii(c: char) {
        let user_ret = unsafe {
            libft::ft_isascii(c as c_int)
        };
        let libc_ret = c_int::from(c.is_ascii());

        assert_nzero!(user_ret, libc_ret);
    }
);

fork_test! {
    #[test]
    fn all_chars() {
        for c in 0..=255_u8 {
            test(c as char)
        }
    }
}
