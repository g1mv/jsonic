use std::fmt::{Display, Formatter, Result};
use std::str::from_utf8;

const EXTRACT_PADDING: usize = 8;

#[derive(Debug)]
pub struct JsonError {
    pub index: usize,
    pub extract: Option<String>,
}

impl JsonError {
    pub fn new(bytes: &[u8], index: usize) -> Self {
        let extract = match from_utf8(&bytes[isize::max(0, index as isize - EXTRACT_PADDING as isize) as usize..usize::min(bytes.len(), index + EXTRACT_PADDING)]) {
            Ok(extract) => { Some(extract.to_owned()) }
            Err(_) => { None }
        };
        return JsonError {
            index,
            extract,
        };
    }
}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.extract {
            Some(extract) => { write!(f, "JSON error near '{}': index {} in data", extract, self.index) }
            None => { write!(f, "JSON error at index {} in data", self.index) }
        }
    }
}