use std::{
    borrow::Cow,
    ffi::CStr,
    marker::PhantomData,
    ops::{Deref, Index},
};

mod functions;
pub use functions::*;

use libc::{c_char, c_void};

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

#[repr(C)]
pub struct TListNode {
    pub data: *mut c_void,
    pub next: *mut TListNode,
}

impl TListNode {
    pub fn new_opt(data: *mut c_void) -> Option<*mut TListNode> {
        unsafe { ft_lstnew(data) }
    }

    pub fn new_panicking(data: *mut c_void) -> *mut TListNode {
        unsafe { ft_lstnew(data).expect("ft_lstnew shouldn't return NULL") }
    }

    pub fn new(data: *mut c_void) -> *mut TListNode {
        unsafe { ft_lstnew(data).unwrap_or(std::ptr::null_mut()) }
    }
}

pub struct TListHandle {
    destroyer: Option<unsafe extern "C" fn(*mut c_void)>,
    front: *mut TListNode,
}

impl TListHandle {
    pub fn new(
        front: *mut TListNode,
        destroyer: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> Self {
        Self { front, destroyer }
    }

    pub fn len(&self) -> usize {
        (unsafe { ft_lstsize(self.front) }) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn remove_front(&mut self) {
        if self.is_empty() {
            return;
        }
        unsafe {
            let next = (*self.front).next;
            ft_lstdelone(self.front, self.destroyer);
            self.front = next;
        }
    }

    pub fn add_front(&mut self, data: *mut TListNode) {
        unsafe {
            ft_lstadd_front(&mut self.front, data);
        }
    }

    pub fn add_back(&mut self, data: *mut TListNode) {
        unsafe {
            ft_lstadd_back(&mut self.front, data);
        }
    }

    pub fn clear(&mut self) {
        unsafe { ft_lstclear(&mut self.front, self.destroyer) }
    }

    pub fn last(&self) -> Option<*mut TListNode> {
        unsafe { ft_lstlast(self.front) }
    }

    pub fn iter(&self) -> TListIter {
        TListIter {
            inner: self.front,
            _phantom: PhantomData,
        }
    }

    pub fn set_destroyer(&mut self, destroyer: Option<unsafe extern "C" fn(*mut c_void)>) {
        self.destroyer = destroyer;
    }

    pub fn with_destroyer(mut self, destroyer: Option<unsafe extern "C" fn(*mut c_void)>) -> Self {
        self.set_destroyer(destroyer);
        self
    }

    pub fn front_as_ptr(&self) -> *const TListNode {
        self.front
    }

    pub fn front_as_ptr_mut(&mut self) -> *mut TListNode {
        self.front
    }
}

impl<P> FromIterator<*mut P> for TListHandle {
    fn from_iter<T: IntoIterator<Item = *mut P>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let Some(first_item) = iter.next() else {
            return TListHandle::new(std::ptr::null_mut(), None);
        };
        let ret = TListHandle {
            front: TListNode::new_panicking(first_item.cast()),
            destroyer: None,
        };
        let mut current = ret.front;
        for item in iter {
            unsafe {
                let new_node = TListNode::new_panicking(item.cast());
                (*current).next = new_node;
                current = new_node;
            }
        }
        ret
    }
}

impl Drop for TListHandle {
    fn drop(&mut self) {
        self.clear();
    }
}

pub struct TListIter<'a> {
    inner: *const TListNode,
    _phantom: PhantomData<&'a ()>,
}

impl Iterator for TListIter<'_> {
    type Item = *const c_void;
    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_null() {
            None
        } else {
            let prec = self.inner;
            unsafe {
                self.inner = (*prec).next;
                Some((*prec).data)
            }
        }
    }
}
