use std::collections::BTreeMap;
use std::ops::Index;
use std::slice::Iter;

use crate::json_type::JsonType;
use crate::json_type::JsonType::{JsonArray, Void, JsonFalse, JsonMap, JsonNull, JsonNumber, JsonTrue, JsonEmptyArray, JsonEmptyMap};
use crate::key::Key;
use crate::slice::Slice;

static EMPTY_ITEM: JsonItem = JsonItem::empty();

#[derive(Debug)]
pub struct JsonItem<'a> {
    pub slice: Slice<'a>,
    pub json_type: JsonType,
    pub array: Option<Vec<JsonItem<'a>>>,
    pub map: Option<BTreeMap<Key, JsonItem<'a>>>,
}

impl<'a> JsonItem<'a> {
    pub fn new(slice: Slice<'a>, json_type: JsonType) -> Self {
        JsonItem { slice, json_type, array: None, map: None }
    }

    pub fn new_array(slice: Slice<'a>, array: Option<Vec<JsonItem<'a>>>) -> Self {
        JsonItem { slice, json_type: if array.is_some() { JsonArray } else { JsonEmptyArray }, array, map: None }
    }

    pub fn new_map(slice: Slice<'a>, map: Option<BTreeMap<Key, JsonItem<'a>>>) -> Self {
        JsonItem { slice, json_type: if map.is_some() { JsonMap } else { JsonEmptyMap }, array: None, map }
    }

    pub const fn empty() -> Self {
        JsonItem { slice: Slice::empty(), json_type: Void, array: None, map: None }
    }

    pub fn as_str(&'a self) -> Option<&'a str> {
        self.slice.as_str()
    }

    pub fn as_f64(&'a self) -> Option<f64> {
        if self.json_type != JsonNumber {
            None
        } else {
            self.as_str()?.parse::<f64>().ok()
        }
    }

    pub fn as_i128(&'a self) -> Option<i128> {
        if self.json_type != JsonNumber {
            None
        } else {
            self.as_str()?.parse::<i128>().ok()
        }
    }

    pub fn as_bool(&'a self) -> Option<bool> {
        match self.json_type {
            JsonTrue => { Some(true) }
            JsonFalse => { Some(false) }
            _ => { None }
        }
    }

    pub fn is_null(&'a self) -> bool {
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

impl<'a> Index<&'a str> for JsonItem<'a> {
    type Output = JsonItem<'a>;

    fn index(&self, key: &'a str) -> &Self::Output {
        if self.json_type == JsonMap {
            if let Some(map) = &self.map {
                return map.get(&Key::from(key.to_owned())).unwrap_or(&EMPTY_ITEM);
            }
        }
        &EMPTY_ITEM
    }
}

impl<'a> Index<usize> for JsonItem<'a> {
    type Output = JsonItem<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        if self.json_type == JsonArray {
            if let Some(array) = &self.array {
                return array.get(index).unwrap_or(&EMPTY_ITEM);
            }
        }
        &EMPTY_ITEM
    }
}