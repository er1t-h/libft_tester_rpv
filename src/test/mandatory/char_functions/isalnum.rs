use crate::assert_nzero;
use crate::verbose;
use crate::fork_test;

fork_test! {
    #[test]
    fn test() {
        for i in 0..=255 {
            let user_ret = unsafe {
                crate::ft_isalnum(i)
            };
            let libc_ret = unsafe {
                libc::isalnum(i)
            };

            verbose!("Current char: {}", i);
            assert_nzero!(user_ret, libc_ret);
        }
    }
}
