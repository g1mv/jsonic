use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::Shl;

use crate::slice::Slice;

#[inline(always)]
// Hash based on bytes at start/end and array length
pub fn hash(bytes: &[u8]) -> u64 {
    let mut hash = (bytes.len() as u64).shl(48);
    for index in 0..usize::min(bytes.len() / 2, 3) {
        hash += (bytes[index] as u64).shl(index.shl(3));
        let next_index = index + 1;
        hash += (bytes[bytes.len() - next_index] as u64).shl(next_index.shl(3));
    }
    hash
}

#[derive(Debug)]
pub struct Key {
    pub slice: Slice,
    pub hash: u64,
}

impl Key {
    pub fn from(slice: Slice) -> Self {
        let hash = hash(slice.as_bytes());
        Key { slice, hash }
    }

    pub fn as_str(&self) -> &str {
        self.slice.as_str()
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.slice.as_bytes().hash(state);
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