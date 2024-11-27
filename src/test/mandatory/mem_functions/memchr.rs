use crate::{
    libft,
    test::{test, DisplayableSlice},
};
use fake::{Fake, Faker};
use libc::c_void;

test!(
    ft_memchr(buffer: DisplayableSlice<u8>, to_find: u8, size: usize) {
        let buffer = buffer.0;
        let libc_ptr = unsafe {
            libc::memchr(buffer.as_ptr() as *const c_void, to_find as i32, size)
        };
        let user_ptr = unsafe {
            libft::ft_memchr(buffer.as_ptr() as *const c_void, to_find as i32, size)
        };
        match (libc_ptr.is_null(), user_ptr) {
            (true, None) => (),
            (false, Some(user_ptr)) => unsafe {
                if user_ptr == libc_ptr { return; }
                let libc_dist = libc_ptr.offset_from(buffer.as_ptr().cast());
                let user_dist = user_ptr.offset_from(buffer.as_ptr().cast());
                panic!("you found it at index {user_dist}, yet libc found it at index {libc_dist}");
            }
            (false, None) => panic!("libc returned something, but not you"),
            (true, Some(_)) => panic!("you returned something, but not libc"),
        }
    }
);

crate::fork_test!(
    #[test]
    fn random() {
        let buffer = fake::vec![u8; 1..10000];
        test(
            DisplayableSlice(&buffer),
            Faker.fake(),
            (0..buffer.len()).fake(),
        );
    }
);
