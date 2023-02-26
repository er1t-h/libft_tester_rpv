use std::ffi::CString;

use libc::c_void;

#[test]
fn basic() {
	let mut list = std::ptr::null_mut();
	let all_strings = (0..20).map(|x| CString::new(x.to_string()).unwrap()).collect::<Vec<CString>>();
	for i in all_strings.iter() {
		unsafe {
			crate::ft_lstadd_back(&mut list, crate::ft_lstnew(
				i.as_ptr() as *mut c_void
			))
		}
	}
	unsafe { crate::ft_lstiter(list, Some(crate::all_plus_one)) };
	for i in all_strings.iter() {
		let tmp = list;
		list = unsafe { *list }.next;
		let content = unsafe { std::slice::from_raw_parts((*tmp).content as *mut u8, i.as_bytes_with_nul().len()) };
		let string_as_bytes = i.as_bytes_with_nul().iter().map(|x| x.to_ascii_uppercase()).collect::<Vec<u8>>();
		assert_eq!(content, string_as_bytes.as_slice(), "Doesn't match. The list was probably not altered.");
		unsafe { libc::free(tmp.cast()) };
	}
}

#[test]
fn list_as_null() {
	unsafe { crate::ft_lstiter(std::ptr::null_mut(), Some(crate::all_plus_one)) };
}

#[test]
fn function_as_null() {
	let mut list = std::ptr::null_mut();
	let all_strings = (0..20).map(|x| CString::new(x.to_string()).unwrap()).collect::<Vec<CString>>();
	for i in all_strings.iter() {
		unsafe {
			crate::ft_lstadd_back(&mut list, crate::ft_lstnew(
				i.as_ptr() as *mut c_void
			))
		}
	}
	unsafe { crate::ft_lstiter(list, None) };
	for i in all_strings.iter() {
		let tmp = list;
		list = unsafe { *list }.next;
		let content = unsafe { std::slice::from_raw_parts((*tmp).content as *mut u8, i.as_bytes_with_nul().len()) };
		assert_eq!(content, i.as_bytes_with_nul(), "Doesn't match. The list was probably not altered.");
		unsafe { libc::free(tmp.cast()) };
	}
}

#[test]
fn both_as_null() {
	unsafe { crate::ft_lstiter(std::ptr::null_mut(), None) };
}
