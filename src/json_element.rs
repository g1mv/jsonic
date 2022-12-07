use std::borrow::Borrow;
use std::ops::Index;

use crate::json_elements::json_array::JsonArray;
use crate::json_elements::json_number::JsonNumber;
use crate::json_elements::json_object::JsonObject;
use crate::json_types::{JsonNumberType, JsonType};
use crate::slice::Slice;

static EMPTY_ELEMENT: JsonElement = JsonElement::empty();

#[derive(Debug)]
pub struct JsonElement<'a> {
    pub json_type: JsonType,
    pub slice: Slice<'a>,
    pub boolean: bool,
    pub number: JsonNumber,
    pub object: JsonObject<'a>,
    pub array: JsonArray<'a>,
}

impl<'a> JsonElement<'_> {
    pub fn get_type(&self) -> &JsonType {
        return &self.json_type;
    }

    pub fn get_slice(&self) -> &Slice {
        return &self.slice;
    }

    pub fn from_type(json_type: JsonType, slice: Slice<'a>) -> JsonElement<'a> {
        return JsonElement {
            json_type,
            slice,
            boolean: false,
            number: JsonNumber { num_i128: 0, num_f64: 0.0, detected_type: JsonNumberType::JsonInteger },
            object: JsonObject::empty(),
            array: JsonArray::empty(),
        };
    }

    pub fn from_null(slice: Slice<'a>) -> JsonElement<'a> {
        return JsonElement::from_type(JsonType::JsonNull, slice);
    }

    pub fn from_boolean(boolean: bool, slice: Slice<'a>) -> JsonElement<'a> {
        return JsonElement {
            json_type: JsonType::JsonBoolean,
            slice,
            boolean,
            number: JsonNumber { num_i128: 0, num_f64: 0.0, detected_type: JsonNumberType::JsonInteger },
            object: JsonObject::empty(),
            array: JsonArray::empty(),
        };
    }

    // pub fn from_string(slice: Slice) -> Self {
    //     return JsonElement::from_type(JsonType::JsonString, slice);
    // }

    pub fn from_number(number: JsonNumber, slice: Slice<'a>) -> JsonElement<'a> {
        return JsonElement {
            json_type: JsonType::JsonNumber,
            slice,
            boolean: false,
            number,
            object: JsonObject::empty(),
            array: JsonArray::empty(),
        };
    }

    pub fn from_object(object: JsonObject<'a>, slice: Slice<'a>) -> JsonElement<'a> {
        return JsonElement {
            json_type: JsonType::JsonObject,
            slice,
            boolean: false,
            number: JsonNumber::empty(),
            object,
            array: JsonArray::empty(),
        };
    }

    pub fn from_array(array: JsonArray<'a>, slice: Slice<'a>) -> JsonElement<'a> {
        return JsonElement {
            json_type: JsonType::JsonArray,
            slice,
            boolean: false,
            number: JsonNumber::empty(),
            object: JsonObject::empty(),
            array,
        };
    }

    pub fn empty() -> JsonElement<'a> {
        return JsonElement {
            json_type: JsonType::JsonEmpty,
            slice: Slice{
                source: &[],
                beginning: 0,
                end: 0,
            },
            boolean: false,
            number: JsonNumber::empty(),
            object: JsonObject::empty(),
            array: JsonArray::empty(),
        };
    }

    pub fn as_str(&self) -> Option<&str> {
        match self.json_type {
            JsonType::JsonString => {
                return Some(self.get_slice().as_str());
            }
            _ => { None }
        }
    }
}

impl<'a> Index<&'a str> for JsonElement<'a> {
    type Output = JsonElement<'a>;

    fn index(&self, key: &str) -> &Self::Output {
        return match self.json_type {
            JsonType::JsonObject => {
                &self.object.map[key]
            }
            _ => {
                &EMPTY_ELEMENT
            }
        }
    }
}