use std::ffi::CStr;

use libc::{c_char, c_uint, c_void};

use crate::libft::ft_strdup;

pub unsafe extern "C" fn no_free(_data: *mut c_void) {}

pub unsafe extern "C" fn all_plus_one(elt: *mut c_void) {
    let mut elt = elt.cast::<u8>();
    while *elt != 0 {
        let c = *elt;
        if c.is_ascii_digit() {
            *elt = (c - b'0' + 1) % 10 + b'0';
        }
        elt = elt.add(1);
    }
}

pub unsafe extern "C" fn clone_and_all_plus_one(elt: *mut c_void) -> *mut c_void {
    let input = CStr::from_ptr(elt.cast());
    let ptr = ft_strdup(input.as_ptr()).expect("strdup returned NULL").leak();
    all_plus_one(ptr.cast());
    ptr.cast()
}

pub unsafe extern "C" fn times_two(elt: *mut c_void) -> *mut c_void {
    let x = elt as u64;
    (x * 2) as *mut c_void
}

pub unsafe extern "C" fn rotx(i: c_uint, c: c_char) -> c_char {
    let c = c as u8;
    if c.is_ascii_uppercase() {
        (((c - b'A') as c_uint + i) % 26 + b'A' as c_uint) as i8
    } else if c.is_ascii_lowercase() {
        (((c - b'a') as c_uint + i) % 26 + b'a' as c_uint) as i8
    } else {
        c as i8
    }
}

pub unsafe extern "C" fn rotone(_i: c_uint, c: c_char) -> c_char {
    rotx(1, c)
}

pub unsafe extern "C" fn to_num(i: c_uint, c: c_char) -> c_char {
    let c = c as u8;
    if c.is_ascii_uppercase() {
        (((c - b'A') as c_uint + i) % 10 + b'0' as c_uint) as i8
    } else if c.is_ascii_lowercase() {
        (((c - b'a') as c_uint + i) % 10 + b'0' as c_uint) as i8
    } else {
        c as i8
    }
}

pub unsafe extern "C" fn rotx_in_place(i: c_uint, s: *mut c_char) {
    *s = rotx(i, *s)
}

pub unsafe extern "C" fn rotone_in_place(i: c_uint, s: *mut c_char) {
    *s = rotone(i, *s)
}

pub unsafe extern "C" fn to_num_in_place(i: c_uint, s: *mut c_char) {
    *s = to_num(i, *s)
}
