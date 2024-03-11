use std::collections::BTreeMap;
use std::ops::Index;
use std::slice::Iter;
use crate::json_item::Container::{Array, Map};

use crate::json_type::JsonType;
use crate::json_type::JsonType::{Empty, JsonArray, JsonFalse, JsonMap, JsonNull, JsonNumber, JsonTrue};
use crate::key::Key;
use crate::slice::Slice;

static EMPTY_ITEM: JsonItem = JsonItem::empty();

#[derive(Debug)]
enum Container<K, V> {
    Array(Vec<V>),
    Map(BTreeMap<K, V>),
}

#[derive(Debug)]
pub struct JsonItem {
    pub(crate) slice: Slice,
    pub(crate) json_type: JsonType,
    container: Option<Container<Key, JsonItem>>,
}

impl JsonItem {
    pub fn new(slice: Slice, json_type: JsonType) -> Self {
        JsonItem { slice, json_type, container: None }
    }

    pub fn new_array(slice: Slice, array: Option<Vec<JsonItem>>) -> Self {
        match array {
            None => { Self::new(slice, JsonArray) }
            Some(array) => { JsonItem { slice, json_type: JsonArray, container: Some(Array(array)) } }
        }
    }

    pub fn new_map(slice: Slice, map: Option<BTreeMap<Key, JsonItem>>) -> Self {
        match map {
            None => { Self::new(slice, JsonMap) }
            Some(map) => { JsonItem { slice, json_type: JsonMap, container: Some(Map(map)) } }
        }
    }

    pub const fn empty() -> Self {
        JsonItem { slice: Slice::empty(), json_type: Empty, container: None }
    }

    pub fn as_str(&self) -> Option<&str> {
        if self.json_type == Empty {
            None
        } else {
            Some(self.slice.as_str())
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        if self.json_type != JsonNumber {
            None
        } else {
            self.as_str()?.parse::<f64>().ok()
        }
    }

    pub fn as_i128(&self) -> Option<i128> {
        if self.json_type != JsonNumber {
            None
        } else {
            self.as_str()?.parse::<i128>().ok()
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self.json_type {
            JsonTrue => { Some(true) }
            JsonFalse => { Some(false) }
            _ => { None }
        }
    }

    pub fn is_null(&self) -> bool {
        self.json_type == JsonNull
    }

    pub fn exists(&self) -> bool {
        self.json_type != Empty
    }

    pub fn get_type(&self) -> &JsonType {
        &self.json_type
    }

    pub fn elements(&self) -> Option<Iter<JsonItem>> {
        if let Some(container) = &self.container {
            if let Array(array) = container {
                return Some(array.iter());
            }
        }
        None
    }

    pub fn entries(&self) -> Option<std::collections::btree_map::Iter<Key, JsonItem>> {
        if let Some(container) = &self.container {
            if let Map(map) = container {
                return Some(map.iter());
            }
        }
        None
    }
}

impl Index<usize> for JsonItem {
    type Output = JsonItem;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(container) = &self.container {
            if let Array(array) = container {
                return array.get(index).unwrap_or(&EMPTY_ITEM);
            }
        }
        &EMPTY_ITEM
    }
}

impl Index<&str> for JsonItem {
    type Output = JsonItem;

    fn index(&self, key: &str) -> &Self::Output {
        if let Some(container) = &self.container {
            if let Map(map) = container {
                return map.get(&Key::from(Slice::from_str(key))).unwrap_or(&EMPTY_ITEM);
            }
        }
        &EMPTY_ITEM
    }
}