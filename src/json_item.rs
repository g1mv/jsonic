use std::collections::BTreeMap;
use std::ops::Index;
use std::slice::Iter;

use crate::json_type::JsonType;
use crate::json_type::JsonType::{Array, Empty, False, Map, Null, Number, True};
use crate::slice::Slice;

static EMPTY_ITEM: JsonItem = JsonItem::empty();

#[derive(Debug)]
pub struct JsonItem<'a> {
    pub slice: Slice<'a>,
    pub json_type: JsonType,
    pub array: Vec<JsonItem<'a>>,
    pub map: BTreeMap<String, JsonItem<'a>>,
}

impl<'a> JsonItem<'a> {
    pub fn new(slice: Slice<'a>, json_type: JsonType) -> Self {
        JsonItem { slice, json_type, array: Vec::new(), map: BTreeMap::new() }
    }

    pub fn new_array(slice: Slice<'a>, array: Vec<JsonItem<'a>>) -> Self {
        JsonItem { slice, json_type: Array, array, map: BTreeMap::new() }
    }

    pub fn new_map(slice: Slice<'a>, map: BTreeMap<String, JsonItem<'a>>) -> Self {
        JsonItem { slice, json_type: Map, array: Vec::new(), map }
    }

    pub const fn empty() -> Self {
        JsonItem { slice: Slice::empty(), json_type: Empty, array: Vec::new(), map: BTreeMap::new() }
    }

    pub fn as_str(&'a self) -> Option<&'a str> {
        self.slice.as_str()
    }

    pub fn as_f64(&'a self) -> Option<f64> {
        if self.json_type != Number {
            None
        } else {
            self.as_str()?.parse::<f64>().ok()
        }
    }

    pub fn as_i128(&'a self) -> Option<i128> {
        if self.json_type != Number {
            None
        } else {
            self.as_str()?.parse::<i128>().ok()
        }
    }

    pub fn as_bool(&'a self) -> Option<bool> {
        match self.json_type {
            True => { Some(true) }
            False => { Some(false) }
            _ => { None }
        }
    }

    pub fn is_null(&'a self) -> bool {
        self.json_type == Null
    }

    pub fn exists(&self) -> bool {
        self.json_type != Empty
    }

    pub fn elements(&self) -> Option<Iter<JsonItem>> {
        if self.json_type == Array {
            Some(self.array.iter())
        } else {
            None
        }
    }

    pub fn entries(&self) -> Option<std::collections::btree_map::Iter<String, JsonItem>> {
        if self.json_type == Map {
            Some(self.map.iter())
        } else {
            None
        }
    }
}

impl<'a> Index<&'a str> for JsonItem<'a> {
    type Output = JsonItem<'a>;

    fn index(&self, key: &str) -> &Self::Output {
        return match self.json_type {
            Map => { self.map.get(key).unwrap_or(&EMPTY_ITEM) }
            _ => { &EMPTY_ITEM }
        };
    }
}

impl<'a> Index<usize> for JsonItem<'a> {
    type Output = JsonItem<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        return match self.json_type {
            Array => { self.array.get(index).unwrap_or(&EMPTY_ITEM) }
            _ => { &EMPTY_ITEM }
        };
    }
}