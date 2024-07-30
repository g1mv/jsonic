use std::fmt::{Display, Formatter, Result};
use std::str::from_utf8;

const EXTRACT_PADDING: usize = 8;

/// Parsing errors
#[derive(Debug)]
pub struct JsonError {
    index: usize,
    extract: Option<String>,
}

impl JsonError {
    pub(crate) fn new(bytes: &[u8], index: usize) -> Self {
        let extract = match from_utf8(&bytes[isize::max(0, index as isize - EXTRACT_PADDING as isize) as usize..usize::min(bytes.len(), index + EXTRACT_PADDING)]) {
            Ok(extract) => { Some(extract.to_owned()) }
            Err(_) => { None }
        };
        return JsonError {
            index,
            extract,
        };
    }


    /// Get error index (position) in source content
    pub fn get_index(&self) -> usize {
        self.index
    }

    /// Returns an optional text extract near the error index
    pub fn get_extract(&self) -> &Option<String> {
        &self.extract
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

impl std::error::Error for JsonError {}
