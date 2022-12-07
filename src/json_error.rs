use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct JsonError {
    pub index: usize,
    pub extract: String,
    pub text: String,
}

impl JsonError {
    pub fn new(source: &[u8], index: usize) -> Self {
        let text = unsafe { String::from_utf8_unchecked(source.to_vec()) };
        let extract = unsafe { String::from_utf8_unchecked(source[isize::max(0, index as isize - 8) as usize..usize::min(source.len(), index + 8)].to_vec()) };
        return JsonError {
            index,
            extract,
            text,
        };
    }
}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "JSON error near '{}' : index {} in {}", self.extract, self.index, self.text)
    }
}