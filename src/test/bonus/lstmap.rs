use libc::c_void;

crate::fork_test!{
	#[test]
	fn basic() {
		let mut list = std::ptr::null_mut();
		for i in 0_usize..20 {
			unsafe {
				crate::ft_lstadd_back(&mut list, crate::ft_lstnew(
					i as *mut c_void
				))
			}
		}
		let mut mapped_list = unsafe { crate::ft_lstmap(list, Some(crate::times_two), Some(crate::nofree)) };
		assert!(!mapped_list.is_null(), "Function returned NULL.");
		for i in 0_usize..20 {
			let tmp = list;
			let tmp_mapped = mapped_list;
			let content = unsafe { *tmp_mapped }.content as usize;
			list = unsafe { *list }.next;
			mapped_list = unsafe { *mapped_list }.next;
			assert_eq!(content, i as usize * 2, "Doesn't match. The list was probably not altered.");
			unsafe { libc::free(tmp.cast()) };
			unsafe { libc::free(tmp_mapped.cast()) };
		}
	}

	#[test]
	fn list_as_null() {
		let user_ret = unsafe { crate::ft_lstmap(std::ptr::null_mut(), Some(crate::times_two), Some(crate::nofree)) };
		assert!(user_ret.is_null(), "The return value of lstmap(NULL, f, del) is not NULL");
	}

	#[test]
	fn function_as_null() {
		let mut list = std::ptr::null_mut();
		for i in 0..20 {
			unsafe {
				crate::ft_lstadd_back(&mut list, crate::ft_lstnew(
					i as *mut c_void
				))
			}
		}
		let mut mapped_list = unsafe { crate::ft_lstmap(list, None, Some(crate::nofree)) };
		if mapped_list.is_null() {
			crate::verbose!("User handled it by returning NULL");
			return;
		}
		for i in 0..20 {
			let tmp = list;
			let tmp_mapped = mapped_list;
			let content = unsafe { *tmp_mapped }.content as usize;
			list = unsafe { *list }.next;
			mapped_list = unsafe { *mapped_list }.next;
			assert_eq!(content, i as usize * 2, "Doesn't match. The list was probably not altered.");
			unsafe { libc::free(tmp.cast()) };
			unsafe { libc::free(tmp_mapped.cast()) };
		}
	}

	#[test]
	fn del_as_null() {
		let mut list = std::ptr::null_mut();
		for i in 0_usize..20 {
			unsafe {
				crate::ft_lstadd_back(&mut list, crate::ft_lstnew(
					i as *mut c_void
				))
			}
		}
		let mut mapped_list = unsafe { crate::ft_lstmap(list, Some(crate::times_two), None) };
		if mapped_list.is_null() {
			crate::verbose!("lstmap returned NULL. I wouldn't consider it false, although having to give a function that does nothing if you don't want to free (cf crate::nofree) something seems a bit strange.");
			return;
		}
		for i in 0_usize..20 {
			let tmp = list;
			let tmp_mapped = mapped_list;
			let content = unsafe { *tmp_mapped }.content as usize;
			list = unsafe { *list }.next;
			mapped_list = unsafe { *mapped_list }.next;
			assert_eq!(content, i as usize * 2, "Doesn't match. The list was probably not altered.");
			unsafe { libc::free(tmp.cast()) };
			unsafe { libc::free(tmp_mapped.cast()) };
		}
	}
}
