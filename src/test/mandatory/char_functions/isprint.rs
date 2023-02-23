use crate::assert_nzero;
use crate::fork_test;
use crate::verbose;

fork_test! {
    #![rusty_fork(timeout_ms = 1000)]

    #[test]
    fn test() {
        for i in 0..=255 {
            let user_ret = unsafe {
                crate::ft_isprint(i)
            };
            let libc_ret = unsafe {
                libc::isprint(i)
            };

            verbose!("Current char: {}", i);
            assert_nzero!(user_ret, libc_ret);
        }
    }
}
