use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ptr::null;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;

pub(crate) struct Slice {
    pub(crate) ptr: *const u8,
    pub(crate) len: usize,
}

impl Slice {
    pub(crate) fn from_str(source: &str) -> Slice {
        Slice {
            ptr: source.as_ptr(),
            len: source.len(),
        }
    }

    pub(crate) fn from_bytes(bytes: &[u8], start: usize, end: usize) -> Slice {
        Slice {
            ptr: unsafe { bytes.as_ptr().byte_add(start) },
            len: end - start,
        }
    }

    pub const fn empty() -> Self {
        Slice { ptr: null(), len: 0 }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(self.ptr, self.len) }
    }

    pub fn as_str(&self) -> &str {
        unsafe { from_utf8_unchecked(self.as_bytes()) }
    }
}

unsafe impl Sync for Slice {}

impl Debug for Slice {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}