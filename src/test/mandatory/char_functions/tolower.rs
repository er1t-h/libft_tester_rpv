use libc::c_int;
use crate::{fork_test, libft, test::test};
use pretty_assertions::assert_eq;

test!(
    ft_tolower(c: char) {
        let user_ret = unsafe {
            libft::ft_tolower(c as c_int)
        };
        let libc_ret = unsafe {
            libc::tolower(c as c_int)
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
