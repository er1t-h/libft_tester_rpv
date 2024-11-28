use crate::{
    libft::{self, TListHandle},
    test::{test, DisplayableStringSlice},
    utils, RANDOM_REPEAT_NUMBER,
};
use fake::{faker::lorem::ja_jp::Words, Fake};
use libc::c_void;
use pretty_assertions::assert_eq;
use std::ffi::{CStr, CString};

test!(
    ft_lstmap(list: DisplayableStringSlice<&str>) {
        let cstrings: Vec<_> = list.0.iter().map(|&x| CString::new(x).expect("DPS: couldn't create string")).collect();
        let mut cstrings_modified: Vec<_> = list.0.iter().map(|&x| CString::new(x).expect("DPS: couldn't create string")).collect();
        let mut list: TListHandle = cstrings.iter().map(|x| x.as_ptr().cast_mut()).collect();

        let new_list_front = unsafe { libft::ft_lstmap(list.front_as_ptr_mut(), Some(utils::clone_and_all_plus_one), Some(libc::free)) };
        let Some(new_list_front) = new_list_front else {
            panic!("lstmap returned NULL")
        };
        let mut new_list = TListHandle::new(new_list_front.cast(), Some(libc::free));
        cstrings_modified.iter_mut().for_each(|x| unsafe { utils::all_plus_one(x.as_ptr().cast_mut().cast()); } );

        for (i, (libft, libc)) in new_list.iter().map(|x| unsafe { CStr::from_ptr(x.cast()) }).zip(cstrings_modified).enumerate() {
            assert_eq!(libft, libc.as_c_str(), "the elements mismatch at index {i}");
        }

        for (i, (libft, libc)) in list.iter().map(|x| unsafe { CStr::from_ptr(x.cast()) }).zip(cstrings).enumerate() {
            assert_eq!(libft, libc.as_c_str(), "the elements of the original lists mismatch at index {i}");
        }
        new_list.remove_front();
    }
);

crate::fork_test! {
    #[test]
    fn random() {
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            let s: Vec<String> = Words(2..200).fake();
            let input: Vec<_> = s.iter().map(|x| x.as_str()).collect();

            test(DisplayableStringSlice(&input));
        }
    }

    #[test]
    fn list_as_null() {
        unsafe { libft::ft_lstmap(std::ptr::null_mut(), Some(utils::clone_and_all_plus_one), Some(libc::free)) };
    }


    #[test]
    fn free_function_as_null() {
        let all_nb = (0..20).collect::<Vec<u64>>();
        let mut list: TListHandle = all_nb.iter().map(|&x| x as *mut c_void).collect();
        let new_list_front = unsafe { libft::ft_lstmap(list.front_as_ptr_mut(), Some(utils::times_two), None) };
        let Some(new_list_front) = new_list_front else {
            eprintln!("chose to return NULL");
            return;
        };
        let new_list = TListHandle::new(new_list_front, None);

        for (i, (libft, &libc)) in list.iter().map(|x| x as u64).zip(&all_nb).enumerate() {
            assert_eq!(libft, libc, "number at index {i} mismatch");
        }
        for (i, (libft, &libc)) in new_list.iter().map(|x| x as u64).zip(&all_nb).enumerate() {
            assert_eq!(libft, libc * 2, "number at index {i} mismatch in mapped");
        }
    }

    #[test]
    fn function_as_null() {
        let all_strings = (0..20).map(|x| CString::new(x.to_string()).unwrap()).collect::<Vec<CString>>();
        let mut list: TListHandle = all_strings.iter().map(|x| x.as_ptr().cast_mut()).collect();
        let new_list_front = unsafe { libft::ft_lstmap(list.front_as_ptr_mut(), None, None) };
        let Some(new_list_front) = new_list_front else {
            eprintln!("chose to return NULL");
            return;
        };
        let new_list = TListHandle::new(new_list_front, None);

        for (i, (libft, libc)) in list.iter().map(|x| unsafe { CStr::from_ptr(x.cast()) }).zip(&all_strings).enumerate() {
            assert_eq!(libft, libc.as_c_str(), "string at index {i} mismatch");
        }
        for (i, (libft, libc)) in new_list.iter().map(|x| unsafe { CStr::from_ptr(x.cast()) }).zip(&all_strings).enumerate() {
            assert_eq!(libft, libc.as_c_str(), "string at index {i} mismatch in mapped");
        }
    }

    #[test]
    fn all_null() {
        assert!(unsafe { libft::ft_lstmap(std::ptr::null_mut(), None, None) }.is_none(), "should return NULL");
    }
}
