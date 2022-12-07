use std::collections::BTreeMap;
use std::ops::Index;

use crate::json_element::JsonElement;

#[derive(Debug)]
pub struct JsonObject<'a> {
    pub map: BTreeMap<String, JsonElement<'a>>,
}

impl JsonObject<'_> {
    pub fn new() -> Self {
        return JsonObject {
            map: BTreeMap::new()
        };
    }

    pub fn check_for(&self, key: &str) -> Option<&JsonElement> {
        return self.map.get(key);
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, String, JsonElement> {
        return self.map.iter();
    }
}

impl<'a> Index<&str> for JsonObject<'a> {
    type Output = JsonElement<'a>;

    fn index(&self, key: &str) -> &Self::Output {
        return &self.map[key];
    }
}
