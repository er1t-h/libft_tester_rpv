use crate::{
    libft::{self, TListHandle},
    test::{test, DisplayableStringSlice},
    utils, RANDOM_REPEAT_NUMBER,
};
use fake::{faker::lorem::ja_jp::Words, Fake};
use pretty_assertions::assert_eq;
use std::ffi::{CStr, CString};

test!(
    ft_lstiter(list: DisplayableStringSlice<&str>) {
        let cstrings: Vec<_> = list.0.iter().map(|&x| CString::new(x).expect("DPS: couldn't create string")).collect();
        let mut cstrings_modified: Vec<_> = list.0.iter().map(|&x| CString::new(x).expect("DPS: couldn't create string")).collect();
        let mut list: TListHandle = cstrings.iter().map(|x| x.as_ptr().cast_mut()).collect();

        unsafe { libft::ft_lstiter(list.front_as_ptr_mut(), Some(utils::all_plus_one)); }
        cstrings_modified.iter_mut().for_each(|x| unsafe { utils::all_plus_one(x.as_ptr().cast_mut().cast()); } );

        for (i, (libft, libc)) in list.iter().map(|x| unsafe { CStr::from_ptr(x.cast()) }).zip(cstrings_modified).enumerate() {
            assert_eq!(libft, libc.as_c_str(), "the elements mismatch at index {i}");
        }
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
        unsafe { libft::ft_lstiter(std::ptr::null_mut(), Some(utils::all_plus_one)) };
    }

    #[test]
    fn function_as_null() {
        let all_strings = (0..20).map(|x| CString::new(x.to_string()).unwrap()).collect::<Vec<CString>>();
        let mut list: TListHandle = all_strings.iter().map(|x| x.as_ptr().cast_mut()).collect();
        unsafe { libft::ft_lstiter(list.front_as_ptr_mut(), None) };

        for (i, (libft, libc)) in list.iter().map(|x| unsafe { CStr::from_ptr(x.cast()) }).zip(&all_strings).enumerate() {
            assert_eq!(libft, libc.as_c_str(), "string at index {i} mismatch");
        }
    }

    #[test]
    fn both_as_null() {
        unsafe { libft::ft_lstiter(std::ptr::null_mut(), None) };
    }
}
