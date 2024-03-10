use std::fmt::{Display, Formatter, Result};
use std::str::from_utf8_unchecked;

#[derive(Debug)]
pub struct JsonError {
    pub index: usize,
    pub extract: String,
}

impl JsonError {
    pub fn new(bytes: &[u8], index: usize) -> Self {
        let extract = &bytes[isize::max(0, index as isize - 8) as usize..usize::min(bytes.len(), index + 8)];
        return JsonError {
            index,
            extract: unsafe { from_utf8_unchecked(extract) }.to_owned(),
        };
    }
}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "JSON error near '{}' : index {}", self.extract, self.index)
    }
}