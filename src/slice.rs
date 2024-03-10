use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ptr::null;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;

pub struct Slice {
    pub ptr: *const u8,
    pub len: usize,
}

impl Slice {
    pub fn from_str(source: &str) -> Slice {
        return Slice {
            ptr: source.as_ptr(),
            len: source.len(),
        };
    }

    pub fn from_bytes(bytes: &[u8], start: usize, end: usize) -> Slice {
        return Slice {
            ptr: unsafe { bytes.as_ptr().byte_add(start) },
            len: end - start,
        };
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