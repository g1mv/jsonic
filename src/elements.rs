use std::collections::BTreeMap;
use std::ops::Index;
use std::slice::Iter;

pub struct Slice {
    pub beginning: usize,
    pub end: usize,
}

impl Slice {
    #[inline(always)]
    pub fn from(beginning: usize, end: usize) -> Self {
        return Slice {
            beginning,
            end,
        };
    }

    // #[inline(always)]
    // pub fn empty() -> Self {
    //     return Slice {
    //         beginning: 0,
    //         end: 0,
    //     };
    // }

    // pub fn as_str<'a>(&self, source: &'a str) -> &'a str {
    //     return unsafe { std::str::from_utf8_unchecked(&source.as_bytes()[self.beginning..self.end]) };
    // }
}

#[derive(Debug, PartialEq)]
pub enum JsonType {
    JsonNull,
    JsonBoolean,
    JsonString,
    JsonNumber,
    JsonObject,
    JsonArray,
}

#[derive(PartialEq)]
pub enum NumberType {
    Float,
    Integer,
}

pub struct Number {
    pub(crate) num_i128: i128,
    pub(crate) num_f64: f64,
    pub(crate) _detected_type: NumberType,
}

impl Number {
    pub fn i128(&self) -> i128 {
        return self.num_i128;
    }

    pub fn f64(&self) -> f64 {
        return self.num_f64;
    }
}

pub struct Object {
    pub map: BTreeMap<&'static str, JsonElement>,
    // pub map: HashMap<String, JsonElement, BuildHasherDefault<fnv::FnvHasher>>,
}

impl Object {
    pub fn new() -> Self {
        return Object {
            map: BTreeMap::new()
            // map: HashMap::with_capacity_and_hasher(8, BuildHasherDefault::<fnv::FnvHasher>::default())
        };
    }

    fn empty() -> Self {
        return Object {
            map: BTreeMap::new()
            // map: HashMap::with_capacity_and_hasher(0, BuildHasherDefault::<fnv::FnvHasher>::default())
        };
    }

    pub fn check_for(&self, key: &str) -> Option<&JsonElement> {
        return self.map.get(key);
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, &'static str, JsonElement> {
        return self.map.iter();
    }
}

impl Index<&str> for Object {
    type Output = JsonElement;

    fn index(&self, key: &str) -> &Self::Output {
        return &self.map[key];
    }
}

pub struct Array {
    pub vec: Vec<JsonElement>,
}

impl Array {
    pub fn new() -> Self {
        return Array {
            vec: Vec::with_capacity(4)
        };
    }

    fn empty() -> Self {
        return Array {
            vec: Vec::with_capacity(0)
        };
    }

    pub fn iter(&self) -> Iter<'_, JsonElement> {
        return self.vec.iter();
    }

    // pub fn len(&self) -> usize {
    //     return self.vec.len();
    // }
}

impl Index<usize> for Array {
    type Output = JsonElement;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.vec[index];
    }
}

pub struct JsonElement {
    pub json_type: JsonType,
    pub slice: Slice,
    pub boolean: bool,
    pub number: Number,
    pub object: Object,
    pub array: Array,
}

impl JsonElement {
    pub fn get_type(&self) -> &JsonType {
        return &self.json_type;
    }

    pub fn get_slice(&self) -> &Slice {
        return &self.slice;
    }

    pub fn from_type(json_type: JsonType, slice: Slice) -> Self {
        return JsonElement {
            json_type,
            slice,
            boolean: false,
            number: Number { num_i128: 0, num_f64: 0.0, _detected_type: NumberType::Integer },
            object: Object::empty(),
            array: Array::empty(),
        };
    }

    pub fn from_null(slice: Slice) -> Self {
        return JsonElement::from_type(JsonType::JsonNull, slice);
    }

    pub fn from_boolean(boolean: bool, slice: Slice) -> Self {
        return JsonElement {
            json_type: JsonType::JsonBoolean,
            slice,
            boolean,
            number: Number { num_i128: 0, num_f64: 0.0, _detected_type: NumberType::Integer },
            object: Object::empty(),
            array: Array::empty(),
        };
    }

    // pub fn from_string(slice: Slice) -> Self {
    //     return JsonElement::from_type(JsonType::JsonString, slice);
    // }

    pub fn from_number(number: Number, slice: Slice) -> Self {
        return JsonElement {
            json_type: JsonType::JsonNumber,
            slice,
            boolean: false,
            number,
            object: Object::empty(),
            array: Array::empty(),
        };
    }

    pub fn from_object(object: Object, slice: Slice) -> Self {
        return JsonElement {
            json_type: JsonType::JsonObject,
            slice,
            boolean: false,
            number: Number { num_i128: 0, num_f64: 0.0, _detected_type: NumberType::Integer },
            object,
            array: Array::empty(),
        };
    }

    pub fn from_array(array: Array, slice: Slice) -> Self {
        return JsonElement {
            json_type: JsonType::JsonArray,
            slice,
            boolean: false,
            number: Number { num_i128: 0, num_f64: 0.0, _detected_type: NumberType::Integer },
            object: Object::empty(),
            array,
        };
    }
}