use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Shl, Shr};

use crate::slice::Slice;

#[inline(always)]
// Hash based on bytes at start/end, mid and array length
fn hash(bytes: &[u8]) -> u64 {
    let mut hash = (bytes.len() as u64).shl(56);
    let mid = bytes.len().shr(1);
    for index in 0..usize::min(mid, 3) {
        let shift = index.shl(4);
        hash += (bytes[index] as u64).shl(shift);
        hash += (bytes[bytes.len() - (index + 1)] as u64).shl(shift + 8_usize);
    }
    if bytes.len() & 0x1 != 0 || bytes.len() & 0xfffffffffffffff8 != 0 {
        hash += (bytes[mid] as u64).shl(48);
    }
    hash
}

/// A struct representing JSON object keys
#[derive(Debug)]
pub struct Key {
    pub(crate) slice: Slice,
    pub(crate) hash: u64,
}

impl Key {
    /// Create a key using &str
    ///
    /// # Arguments
    /// * `source` - Key id
    pub fn from_str(source: &str) -> Self {
        Self::from_slice(Slice::from_str(source))
    }

    pub(crate) fn from_slice(slice: Slice) -> Self {
        let hash = hash(slice.as_bytes());
        Key { slice, hash }
    }

    ///
    pub fn as_str(&self) -> &str {
        self.slice.as_str()
    }
}

impl Eq for Key {}

impl PartialEq<Self> for Key {
    fn eq(&self, other: &Self) -> bool {
        if self.hash != other.hash {
            false
        } else {
            self.as_str().eq(other.as_str())
        }
    }
}

impl PartialOrd<Self> for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hash == other.hash {
            self.as_str().cmp(other.as_str())
        } else {
            self.hash.cmp(&other.hash)
        }
    }
}