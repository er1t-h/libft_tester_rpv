use libc::c_int;

use crate::{assert_nzero, fork_test, libft, test::test};

test!(
    ft_isalpha(c: char) {
        let user_ret = unsafe {
            libft::ft_isalpha(c as c_int)
        };
        let libc_ret = unsafe {
            libc::isalpha(c as c_int)
        };

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
