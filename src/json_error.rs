use std::fmt::{Display, Formatter, Result};
use std::str::from_utf8;

const SOURCE_SIZE: usize = 8192;
const EXTRACT_PADDING: isize = 16;

#[derive(Debug)]
pub struct JsonError {
    pub index: usize,
    pub extract: Option<String>,
    pub source: Option<String>,
}

impl JsonError {
    pub fn new(bytes: &[u8], index: usize) -> Self {
        let source = from_utf8(&bytes[0..usize::min(bytes.len(), SOURCE_SIZE)]);
        let extract = from_utf8(&bytes[isize::max(0, index as isize - EXTRACT_PADDING) as usize..usize::min(bytes.len(), index + EXTRACT_PADDING as usize)]);
        return JsonError {
            index,
            extract: match extract {
                Ok(extract) => { Some(extract.to_owned()) }
                Err(_) => { None }
            },
            source: match source {
                Ok(source) => { if bytes.len() <= SOURCE_SIZE { Some(source.to_owned()) } else { Some(format!("{}...", source)) } }
                Err(_) => { None }
            },
        };
    }
}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "JSON error near '{}': index {} in {}", &self.extract.as_ref().unwrap_or(&String::from("?")), self.index, self.source.as_ref().unwrap_or(&String::from("?")))
    }
}