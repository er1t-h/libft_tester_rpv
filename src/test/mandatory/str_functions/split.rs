use crate::verbose;
use regex::Regex;
use std::ffi::CString;

macro_rules! test {
	($name: ident, $str: expr, $delimiter: expr) => {
		crate::fork_test!{
			#[test]
			fn $name() {
				let re = Regex::new(&($delimiter.to_string() + "+")).unwrap();
				let cleaned_str = re.replace_all($str.trim_matches($delimiter), $delimiter.to_string());
				let str = CString::new($str).unwrap();
				verbose!("cleaned_str = {}", cleaned_str);
				let splitted_str = cleaned_str.split($delimiter);
				let table_size = splitted_str.clone().count();
				verbose!("Before split");
				let user_ret = unsafe {
					crate::ft_split(str.as_ptr(), $delimiter as i8)
				};
				verbose!("After split");
				verbose!("Table size: {}", table_size);
				let returned_table = unsafe {
					std::slice::from_raw_parts(user_ret, table_size + 1)
				};
				for (index, str) in splitted_str.enumerate() {
					let current_ptr = returned_table[index] as *mut u8;
					let content = unsafe { std::slice::from_raw_parts(current_ptr, str.len() + 1) };
					assert_eq!(str.as_bytes(), &content[..content.len() - 1], "String content do not match");
					assert_eq!(content[content.len() - 1], 0, "String is not null terminated");
					unsafe { libc::free(current_ptr as *mut libc::c_void) };
				}
				assert!(returned_table[table_size].is_null(), "Array is not NULL terminated");
				unsafe { libc::free(user_ret as *mut libc::c_void) };
			}
		}
	};
}

test!(basic, "C'est un super test, n'est-ce pas ?", ' ');
test!(no_split, "C'est un super test, n'est-ce pas ?", '!');
test!(
    only_internal_delimiter,
    "C'est      un         super       test,        n'est-ce        pas       ?",
    ' '
);
test!(only_external_delimiter, "!!!!!!!!!!!!!!!!!!!!!!!!!!!C'estunsupertest,n'est-cepas?!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!", '!');
test!(many_split, "!a!a!a!a!a!a!a!a!a!a!a!a!a!a!a!a!a!a!a!a!a!a!a!a!a!a!aC!'!estun!sup!!!ertest,n!'!!es!!!!t-cep!!!as?!!!!!fe!!!!!!!!!!!!wqd!!!!!!!!fqef!!!!!!qef!!!!!!!!", '!');

crate::fork_test! {
    #[test]
    fn null() {
        let user_ret = unsafe {
            crate::ft_split(std::ptr::null(), 'v' as i8)
        };
        if user_ret.is_null() {
            verbose!("User chose to return NULL when NULL is given to split");
            return;
        }
        if unsafe { *user_ret }.is_null() {
			unsafe { libc::free(user_ret.cast()) };
            verbose!("User chose to allocate an empty array when NULL is given to split");
            return;
        }
        // If you go through here, you handled passing NULL in a way I didn't anticipate.
        // If you can explain it clearly, you may ignore this failing test.
        // However, if you let your code crash with this test
        // because you didn't handle having NULL as an argument, you should get
        // a Crash flag.
        panic!("Handled passing NULL to split in a strange way");
    }
}
