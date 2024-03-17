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

/// Container for a JSON element, i.e. can contain a JSON null, bool, string, number, object or array.
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

    const fn empty() -> Self {
        JsonItem { slice: Slice::empty(), json_type: Empty, container: None }
    }

    /// Returns &str value of item.
    /// This only returns None if the item is non-existent.
    pub fn as_str(&self) -> Option<&str> {
        if self.json_type == Empty {
            None
        } else {
            Some(self.slice.as_str())
        }
    }

    /// Tries to convert item to f64. If the conversion fails, returns None.
    pub fn as_f64(&self) -> Option<f64> {
        if self.json_type != JsonNumber {
            None
        } else {
            self.as_str()?.parse::<f64>().ok()
        }
    }

    /// Tries to convert item to an i128 integer. If the conversion fails, returns None.
    /// Resulting i128 can then be easily converted to other integer types using "as".
    pub fn as_i128(&self) -> Option<i128> {
        if self.json_type != JsonNumber {
            None
        } else {
            self.as_str()?.parse::<i128>().ok()
        }
    }

    /// Tries to convert item to a bool. If the conversion fails, returns None.
    pub fn as_bool(&self) -> Option<bool> {
        match self.json_type {
            JsonTrue => { Some(true) }
            JsonFalse => { Some(false) }
            _ => { None }
        }
    }

    /// Checks if item is a JSON null
    pub fn is_null(&self) -> bool {
        self.json_type == JsonNull
    }

    /// Tests if item exists
    pub fn exists(&self) -> bool {
        self.json_type != Empty
    }

    /// Returns item's type
    pub fn get_type(&self) -> &JsonType {
        &self.json_type
    }

    /// If the item is an array, returns an iterator over array elements. Otherwise, returns None.
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

    /// If the item is an object, returns an iterator over object entries. Otherwise, returns None.
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