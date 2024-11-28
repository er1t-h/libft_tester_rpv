use crate::{fork_test, libft, test::test};
use libc::c_int;
use pretty_assertions::assert_eq;

test!(
    ft_toupper(c: char) {
        let user_ret = unsafe {
            libft::ft_toupper(c as c_int)
        };
        let libc_ret = unsafe {
            libc::toupper(c as c_int)
        };

        assert_eq!(user_ret, libc_ret, "character mismatch");
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
