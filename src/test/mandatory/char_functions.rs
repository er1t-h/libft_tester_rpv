use rusty_fork::rusty_fork_test;
use crate::assert_nzero;
use crate::verbose;

rusty_fork_test! {
	#![rusty_fork(timeout_ms = 100)]

	#[test]
	fn isalnum() {
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

	#[test]
	fn isalpha() {
		for i in 0..=255 {
			let user_ret = unsafe {
				crate::ft_isalpha(i)
			};
			let libc_ret = unsafe {
				libc::isalpha(i)
			};

			verbose!("Current char: {}", i);
			assert_nzero!(user_ret, libc_ret);
		}
	}

	#[test]
	fn isascii() {
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

	#[test]
	fn isdigit() {
		for i in 0..=255 {
			let user_ret = unsafe {
				crate::ft_isdigit(i)
			};
			let libc_ret = unsafe {
				libc::isdigit(i)
			};

			verbose!("Current char: {}", i);
			assert_nzero!(user_ret, libc_ret);
		}
	}

	#[test]
	fn isprint() {
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

	#[test]
	fn tolower() {
		for i in 0..=255 {
			let user_ret = unsafe {
				crate::ft_tolower(i)
			};
			let libc_ret = unsafe {
				libc::tolower(i)
			};

			verbose!("Current char: {}", i);
			assert_eq!(user_ret, libc_ret);
		}
	}

	#[test]
	fn toupper() {
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

