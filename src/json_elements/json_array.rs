use std::ops::Index;
use std::slice::Iter;
use crate::json_element::JsonElement;

#[derive(Debug)]
pub struct JsonArray<'a> {
    pub vec: Vec<JsonElement<'a>>,
}

impl JsonArray<'_> {
    pub fn new() -> Self {
        return JsonArray {
            vec: Vec::with_capacity(4)
        };
    }

    pub fn iter(&self) -> Iter<JsonElement> {
        return self.vec.iter();
    }
}

impl<'a> Index<usize> for JsonArray<'a> {
    type Output = JsonElement<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.vec[index];
    }
}