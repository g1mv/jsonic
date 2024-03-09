use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct JsonError {
    pub index: usize,
    pub extract: String,
    pub text: String,
}

impl JsonError {
    pub fn new(source: &str, index: usize) -> Self {
        let extract = &source[isize::max(0, index as isize - 8) as usize..usize::min(source.len(), index + 8)];
        return JsonError {
            index,
            extract: extract.to_owned(),
            text: source.to_owned(),
        };
    }
}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "JSON error near '{}' : index {} in {}", self.extract, self.index, self.text)
    }
}