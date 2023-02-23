use crate::verbose;
use crate::fork_test;
#[cfg(feature = "fork")]
use rusty_fork::rusty_fork_test;

fork_test!{
    #[test]
    fn test() {
        for i in 0..=255 {
            let user_ret = unsafe {
                crate::ft_toupper(i)
            };
            let libc_ret = unsafe {
                libc::toupper(i)
            };

            verbose!("Current char: {}", i);
            assert_eq!(user_ret, libc_ret);
        }
    }
}
