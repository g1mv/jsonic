#[derive(Debug)]
pub struct JsonError {
    pub index: usize,
}

impl JsonError {
    pub fn new(index: usize) -> Self {
        return JsonError {
            index
        };
    }
}