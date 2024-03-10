use std::fmt;
use std::fmt::{Debug, Formatter};
use std::str::from_utf8_unchecked;

pub struct Slice<'a> {
    pub bytes: &'a [u8],
    pub start: usize,
    pub end: usize,
}

impl<'a> Slice<'a> {
    pub fn new(bytes: &'a [u8], start: usize, end: usize) -> Slice<'a> {
        return Slice {
            bytes,
            start,
            end,
        };
    }

    pub const fn empty() -> Self {
        Slice { bytes: &[], start: 0, end: 0 }
    }

    pub fn as_str(&self) -> Option<&str> {
        if self.end != 0 {
            Some(unsafe { from_utf8_unchecked(&self.bytes[self.start..self.end]) })
        } else {
            None
        }
    }
}

impl Debug for Slice<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}