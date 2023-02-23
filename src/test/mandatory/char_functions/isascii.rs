use crate::fork_test;
use crate::verbose;

fork_test!{
    #![rusty_fork(timeout_ms = 1000)]

    #[test]
    fn test() {
        for i in 0..=255 {
            let user_ret = unsafe {
                crate::ft_isascii(i)
            };
            // The libc binding don't provide isascii, so I'm using rust's std
            // function. It works exactly the same and won't be a problem.
            let libc_ret = (i as u8 as char).is_ascii();

            verbose!("Current char: {}", i);
            assert_eq!(user_ret != 0, libc_ret);
        }
    }
}
