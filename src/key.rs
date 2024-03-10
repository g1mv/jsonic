use std::cmp::Ordering;

#[derive(Debug)]
pub struct Key {
    pub key: String,
}

impl Key {
    pub fn from(key: String) -> Self {
        Key { key }
    }
}

impl Eq for Key {}

impl PartialEq<Self> for Key {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl PartialOrd<Self> for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}