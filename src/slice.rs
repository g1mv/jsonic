use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct Slice<'a> {
    pub source: &'a str,
    pub start: usize,
    pub end: usize,
}

impl<'a> Slice<'a> {
    pub fn new(source: &'a str, start: usize, end: usize) -> Slice<'a> {
        return Slice {
            source,
            start,
            end,
        };
    }

    pub const fn empty() -> Self {
        Slice { source: "", start: 0, end: 0 }
    }

    pub fn as_str(&self) -> Option<&str> {
        if self.end != 0 {
            Some(&self.source[self.start..self.end])
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