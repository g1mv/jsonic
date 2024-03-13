use std::ops::Index;

use crate::generics::{ArrayIterator, Container, MapIterator};
use crate::generics::Container::{Array, MapBTree, MapVec};
use crate::generics::IterArray::{IterArrayEmpty, IterArrayVec};
use crate::generics::IterMap::{IterMapBTree, IterMapEmpty, IterMapVec};
use crate::json_type::JsonType;
use crate::json_type::JsonType::{Empty, JsonArray, JsonFalse, JsonMap, JsonNull, JsonNumber, JsonTrue};
use crate::key::Key;
use crate::slice::Slice;

const KEEP_VEC_THRESHOLD: usize = 64;

static EMPTY_ITEM: JsonItem = JsonItem::empty();

#[derive(Debug)]
pub struct JsonItem {
    pub(crate) slice: Slice,
    pub(crate) json_type: JsonType,
    container: Option<Container<Key, JsonItem>>,
}

impl JsonItem {
    pub(crate) fn new(slice: Slice, json_type: JsonType) -> Self {
        JsonItem { slice, json_type, container: None }
    }

    pub(crate) fn new_array(slice: Slice, array: Option<Vec<JsonItem>>) -> Self {
        match array {
            None => { Self::new(slice, JsonArray) }
            Some(array) => { JsonItem { slice, json_type: JsonArray, container: Some(Array(array)) } }
        }
    }

    pub(crate) fn new_map(slice: Slice, map: Option<Vec<(Key, JsonItem)>>) -> Self {
        match map {
            None => { Self::new(slice, JsonMap) }
            Some(map) => {
                let container = if map.len() <= KEEP_VEC_THRESHOLD {
                    MapVec(map)
                } else {
                    MapBTree(map.into_iter().map(|(k, v)| (k, v)).collect())
                };
                JsonItem { slice, json_type: JsonMap, container: Some(container) }
            }
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

    pub fn elements(&self) -> Option<ArrayIterator<JsonItem>> {
        if let Some(container) = &self.container {
            if let Array(array) = container {
                return Some(ArrayIterator { iter: IterArrayVec(array.iter()) });
            }
        } else {
            if self.json_type == JsonArray {
                return Some(ArrayIterator { iter: IterArrayEmpty() });
            }
        }
        None
    }

    pub fn entries(&self) -> Option<MapIterator<Key, JsonItem>> {
        if let Some(container) = &self.container {
            return match container {
                MapVec(map) => { Some(MapIterator { iter: IterMapVec(map.iter()) }) }
                MapBTree(map) => { Some(MapIterator { iter: IterMapBTree(map.iter()) }) }
                _ => { None }
            };
        } else {
            if self.json_type == JsonMap {
                return Some(MapIterator { iter: IterMapEmpty() });
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
        let key = Key::from_slice(Slice::from_str(key));
        if let Some(container) = &self.container {
            match container {
                MapVec(map) => {
                    for (k, v) in map {
                        if key.eq(k) { return v; }
                    }
                }
                MapBTree(map) => {
                    return map.get(&key).unwrap_or(&EMPTY_ITEM);
                }
                _ => {}
            }
        }
        &EMPTY_ITEM
    }
}