use std::{
    borrow::Cow,
    env::current_dir,
    ffi::CStr,
    ops::{Deref, Index},
    sync::LazyLock,
};

use libc::{c_char, c_int, c_uint, c_void, size_t};
use libloading::{Library, Symbol};

pub struct OwnedCString(&'static CStr);

impl OwnedCString {
    pub fn as_c_str(&self) -> &CStr {
        self.0
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr()
    }

    pub fn as_utf8_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(self.to_bytes())
    }

    ///
    /// Returns the raw pointer, not freeing it. Make sure to do it yourself.
    ///
    pub fn leak(self) -> *mut c_char {
        let inner = self.0.as_ptr().cast_mut();
        std::mem::forget(self);
        inner
    }
}

impl Deref for OwnedCString {
    type Target = CStr;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl Drop for OwnedCString {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.0.as_ptr().cast_mut().cast());
        }
    }
}

pub struct SplitTable {
    content: *mut *mut c_char,
    len: usize,
}
impl SplitTable {
    fn new(content: *mut *mut c_char) -> Self {
        assert!(
            !content.is_null(),
            "tried to init split table with null pointer"
        );
        let mut len = 0;
        unsafe {
            let mut current = content;
            while !(*current).is_null() {
                len += 1;
                current = current.add(1);
            }
        }
        Self { content, len }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    ///
    /// Returns the raw pointer, not freeing it. Make sure to do it yourself.
    ///
    pub fn leak(self) -> *mut *mut c_char {
        let inner = self.content;
        std::mem::forget(self);
        inner
    }
}
impl Drop for SplitTable {
    fn drop(&mut self) {
        unsafe {
            let mut current = self.content;
            for _ in 0..self.len {
                libc::free(*current.cast());
                current = current.add(1);
            }
            libc::free(self.content.cast());
        }
    }
}
impl Index<usize> for SplitTable {
    type Output = CStr;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len, "index out of range");
        unsafe {
            let table = self.content;
            CStr::from_ptr(*table.add(index))
        }
    }
}

pub struct OwnedCPointer<T>(*mut T);
impl<T> Drop for OwnedCPointer<T> {
    fn drop(&mut self) {
        unsafe { libc::free(self.0.cast()) };
    }
}
impl<T> OwnedCPointer<T> {
    pub fn as_ptr(&self) -> *const T {
        self.0
    }
    pub fn as_mut_ptr(&self) -> *mut T {
        self.0
    }
    ///
    /// Returns the raw pointer, not freeing it. Make sure to do it yourself.
    ///
    pub fn leak(self) -> *mut T {
        let inner = self.0;
        std::mem::forget(self);
        inner
    }
}

pub static LIBRARY: LazyLock<Library> = LazyLock::new(|| unsafe {
    Library::new(format!(
        "{}/libft.so",
        current_dir()
            .expect("DPS: couldn't find the current directory")
            .display()
    ))
    .expect("DPS: couldn't load the dynamic library")
});

macro_rules! function_wrapper {
    () => {};
    ($function_name: ident ($($param_name: tt: $param_type: ty),+ $(,)?) -> SplitTable; $($rest: tt)*) => {
        pub unsafe fn $function_name($($param_name: $param_type),+) -> Option<SplitTable> {
            let name = stringify!($function_name);
            let f: Symbol<unsafe fn($($param_name: $param_type),+) -> *mut *mut c_char> = LIBRARY.deref().get(name.as_bytes()).expect("function doesn't exist");
            let ptr = f($($param_name),+);
            if ptr.is_null() {
                None
            } else {
                Some(SplitTable::new(ptr))
            }
        }
        function_wrapper!($($rest)*);
    };
    ($function_name: ident ($($param_name: tt: $param_type: ty),+ $(,)?) -> OwnedCPointer<$return_type: ty>; $($rest: tt)*) => {
        pub unsafe fn $function_name($($param_name: $param_type),+) -> Option<OwnedCPointer<$return_type>> {
            let name = stringify!($function_name);
            let f: Symbol<unsafe fn($($param_name: $param_type),+) -> *mut $return_type> = LIBRARY.deref().get(name.as_bytes()).expect("function doesn't exist");
            let ptr = f($($param_name),+);
            if ptr.is_null() {
                None
            } else {
                Some(OwnedCPointer(ptr))
            }
        }
        function_wrapper!($($rest)*);
    };
    ($function_name: ident ($($param_name: tt: $param_type: ty),+ $(,)?) -> OwnedCString; $($rest: tt)*) => {
        pub unsafe fn $function_name($($param_name: $param_type),+) -> Option<OwnedCString> {
            let name = stringify!($function_name);
            let f: Symbol<unsafe fn($($param_name: $param_type),+) -> *mut c_char> = LIBRARY.deref().get(name.as_bytes()).expect("function doesn't exist");
            let ptr = f($($param_name),+);
            if ptr.is_null() {
                None
            } else {
                Some(OwnedCString(CStr::from_ptr(ptr)))
            }
        }
        function_wrapper!($($rest)*);
    };
    ($function_name: ident ($($param_name: tt: $param_type: ty),+ $(,)?) -> CStr; $($rest: tt)*) => {
        pub unsafe fn $function_name($($param_name: $param_type),+) -> Option<&'static CStr> {
            let name = stringify!($function_name);
            let f: Symbol<unsafe fn($($param_name: $param_type),+) -> *mut c_char> = LIBRARY.deref().get(name.as_bytes()).expect("function doesn't exist");
            let ptr = f($($param_name),+);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
        function_wrapper!($($rest)*);
    };
    ($function_name: ident ($($param_name: tt: $param_type: ty),+ $(,)?) -> *mut $return_type: ty; $($rest: tt)*) => {
        pub unsafe fn $function_name($($param_name: $param_type),+) -> Option<*mut $return_type> {
            let name = stringify!($function_name);
            let f: Symbol<unsafe fn($($param_name: $param_type),+) -> *mut $return_type> = LIBRARY.deref().get(name.as_bytes()).expect("function doesn't exist");
            let ptr = f($($param_name),+);
            if ptr.is_null() {
                None
            } else {
                Some(ptr)
            }
        }
        function_wrapper!($($rest)*);
    };
    ($function_name: ident ($($param_name: tt: $param_type: ty),+ $(,)?) $(-> $return_type: ty)?; $($rest: tt)*) => {
        pub unsafe fn $function_name($($param_name: $param_type),+) $(-> $return_type)? {
            let name = stringify!($function_name);
            let f: Symbol<unsafe fn($($param_name: $param_type),+) $(-> $return_type)?> = LIBRARY.deref().get(name.as_bytes()).expect("function doesn't exist");
            f($($param_name),+)
        }
        function_wrapper!($($rest)*);
    };
}

#[repr(C)]
pub struct TList {
    pub data: *mut c_void,
    pub next: *mut TList,
}

function_wrapper! {
    ft_isalpha(c: c_int) -> c_int;
    ft_isdigit(c: c_int) -> c_int;
    ft_isalnum(c: c_int) -> c_int;
    ft_isascii(c: c_int) -> c_int;
    ft_isprint(c: c_int) -> c_int;

    ft_toupper(c: c_int) -> c_int;
    ft_tolower(c: c_int) -> c_int;

    ft_strlen(buffer: *const c_char) -> size_t;
    ft_strlcpy(dest: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    ft_strlcat(dest: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    ft_strncmp(str1: *const c_char, str2: *const c_char, n: size_t) -> c_int;
    ft_split(s: *const c_char, c: c_char) -> SplitTable;
    ft_striteri(s: *mut c_char, f: Option<unsafe extern "C" fn(c_uint, *mut c_char)>);

    ft_atoi(s: *const c_char) -> c_int;

    ft_putchar_fd(c: c_char, fd: c_int);
    ft_putendl_fd(s: *const c_char, fd: c_int);
    ft_putstr_fd(s: *const c_char, fd: c_int);
    ft_putnbr_fd(n: c_int, fd: c_int);

    ft_bzero(mem: *mut c_void, n: size_t);
    ft_calloc(nb_element: size_t, size_of_element: size_t) -> OwnedCPointer<c_void>;
    ft_memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    ft_memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    ft_memchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    ft_memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    ft_memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;

    ft_lstnew(content: *mut c_void) -> *mut TList;
    ft_lstadd_back(alst: *mut *mut TList, new: *mut TList);
    ft_lstadd_front(alst: *mut *mut TList, new: *mut TList);
    ft_lstclear(lst: *mut *mut TList, del: Option<unsafe extern "C" fn(*mut c_void)>);
    ft_lstdelone(lst: *mut TList, del: Option<unsafe extern "C" fn(*mut c_void)>);
    ft_lstsize(list: *mut TList) -> c_int;
    ft_lstlast(list: *mut TList) -> *mut TList;

    ft_lstiter(list: *mut TList, f: Option<unsafe extern "C" fn(*mut c_void)>);
    ft_strchr(s1: *const c_char, c: c_int) -> CStr;
    ft_strrchr(s1: *const c_char, c: c_int) -> CStr;
    ft_strnstr(haystack: *const c_char, needle: *const c_char, len: size_t) -> CStr;
    ft_strdup(s: *const c_char) -> OwnedCString;
    ft_strjoin(s1: *const c_char, s2: *const c_char) -> OwnedCString;
    ft_substr(s: *const c_char, start: c_uint, len: size_t) -> OwnedCString;
    ft_strtrim(s1: *const c_char, set: *const c_char) -> OwnedCString;
    ft_strmapi(s: *const c_char, f: Option<unsafe extern "C" fn(c_uint, c_char) -> c_char>) -> OwnedCString;
    ft_lstmap(
        list: *mut TList,
        f: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        del: Option<unsafe extern "C" fn(*mut c_void)>
    );
}
