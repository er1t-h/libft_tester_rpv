use std::ffi::{CStr, CString};

use fake::Fake;

use crate::{libft::{TListHandle, TListNode}, test::{test, DisplayableStringSlice}, RANDOM_REPEAT_NUMBER};

test!(
    ft_lstadd_back(elements: DisplayableStringSlice<&str>, pre_add: usize) {
        let elements: Vec<_> = elements.0.iter().map(|&x| CString::new(x).expect("DPS: couldn't create string")).collect();
        let (begin, end) = elements.split_at(pre_add);
        let mut list: TListHandle = begin.iter().map(|x| x.as_ptr().cast_mut()).collect();

        debug_assert_eq!(list.len(), begin.len(), "DPS: FromIter didn't work");

        for item in end.iter().map(|x| x.as_ptr().cast_mut().cast()) {
            list.add_back(TListNode::new_panicking(item));
        }

        assert_eq!(list.len(), elements.len(), "the size of the lists mismatch");
        assert_eq!(list.last().map(|x| unsafe { CStr::from_ptr((*x).data.cast()) }), elements.last().map(|x| x.as_c_str()), "last returned the wrong element");

        for (i, (libft, libc)) in list.iter().map(|x| unsafe { CStr::from_ptr(x.cast()) }).zip(elements).enumerate() {
            assert_eq!(libft, libc.as_c_str(), "the content differ at index {i}");
        }
        // We do this to assert that lst_delone frees correctly
        list.remove_front();
    }
);

crate::fork_test!{
    #[test]
    fn random() {
        eprintln!("How to read those tests");
        eprintln!("ft_lstadd_back(`elements`, `element_present_at_start`)");
        eprintln!("The first `element_present_at_start` are already in the list (in the same order lst_addfront would have added them). The remaining elements will be added by lstadd_back");
        for _ in 0..*RANDOM_REPEAT_NUMBER {
            let string_table = fake::vec![String; 1..100];
            let string_slice: Vec<_> = string_table.iter().map(|x| x.as_str()).collect();
            test(DisplayableStringSlice(&string_slice), (0..string_slice.len()).fake());
        }
    }
}