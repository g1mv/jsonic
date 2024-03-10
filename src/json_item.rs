use std::collections::BTreeMap;
use std::ops::Index;
use std::slice::Iter;

use crate::json_type::JsonType;
use crate::json_type::JsonType::{JsonArray, JsonEmptyArray, JsonEmptyMap, JsonFalse, JsonMap, JsonNull, JsonNumber, JsonTrue, Void};
use crate::key::Key;
use crate::slice::Slice;

static EMPTY_ITEM: JsonItem = JsonItem::empty();

#[derive(Debug)]
pub struct JsonItem {
    pub slice: Slice,
    pub json_type: JsonType,
    pub array: Option<Vec<JsonItem>>,
    pub map: Option<BTreeMap<Key, JsonItem>>,
}

impl JsonItem {
    pub fn new(slice: Slice, json_type: JsonType) -> Self {
        JsonItem { slice, json_type, array: None, map: None }
    }

    pub fn new_array(slice: Slice, array: Option<Vec<JsonItem>>) -> Self {
        JsonItem { slice, json_type: if array.is_some() { JsonArray } else { JsonEmptyArray }, array, map: None }
    }

    pub fn new_map(slice: Slice, map: Option<BTreeMap<Key, JsonItem>>) -> Self {
        JsonItem { slice, json_type: if map.is_some() { JsonMap } else { JsonEmptyMap }, array: None, map }
    }

    pub const fn empty() -> Self {
        JsonItem { slice: Slice::empty(), json_type: Void, array: None, map: None }
    }

    pub fn as_str(&self) -> Option<&str> {
        if self.json_type == Void {
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
        self.json_type != Void
    }

    pub fn elements(&self) -> Option<Iter<JsonItem>> {
        if self.json_type == JsonArray {
            if let Some(array) = &self.array {
                return Some(array.iter());
            }
        }
        None
    }

    pub fn entries(&self) -> Option<std::collections::btree_map::Iter<Key, JsonItem>> {
        if self.json_type == JsonMap {
            if let Some(map) = &self.map {
                return Some(map.iter());
            }
        }
        None
    }
}

impl Index<&str> for JsonItem {
    type Output = JsonItem;

    fn index(&self, key: &str) -> &Self::Output {
        if self.json_type == JsonMap {
            if let Some(map) = &self.map {
                return map.get(&Key::from(Slice::from_str(key))).unwrap_or(&EMPTY_ITEM);
            }
        }
        &EMPTY_ITEM
    }
}

impl Index<usize> for JsonItem {
    type Output = JsonItem;

    fn index(&self, index: usize) -> &Self::Output {
        if self.json_type == JsonArray {
            if let Some(array) = &self.array {
                return array.get(index).unwrap_or(&EMPTY_ITEM);
            }
        }
        &EMPTY_ITEM
    }
}