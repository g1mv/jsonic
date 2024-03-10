use std::collections::BTreeMap;

use crate::json_error::JsonError;
use crate::json_item::JsonItem;
use crate::json_type::JsonType::{JsonFalse, JsonNull, JsonNumber, JsonString, JsonTrue};
use crate::key::Key;
use crate::slice::Slice;

pub mod json_error;
pub mod slice;
pub mod json_item;

pub mod json_type;
pub mod key;

const DEFAULT_VEC_CAPACITY: usize = 2;

#[inline(always)]
fn shift_index(item: &JsonItem) -> usize {
    if item.json_type == JsonString {
        item.slice.len + 2
    } else {
        item.slice.len
    }
}

#[inline(always)]
fn skip_spaces(bytes: &[u8], mut index: usize) -> Result<usize, JsonError> {
    while index < bytes.len() {
        match bytes[index] {
            b' ' | b'\n' | b'\r' | b'\t' => {}
            _ => { return Ok(index); }
        }
        index += 1;
    }
    Err(JsonError::new(bytes, index))
}

#[inline(always)]
fn parse_null(bytes: &[u8], index: usize) -> Result<JsonItem, JsonError> {
    if index + 3 < bytes.len() {
        if bytes[index + 1] == b'u' && bytes[index + 2] == b'l' && bytes[index + 3] == b'l' {
            return Ok(JsonItem::new(Slice::from_bytes(bytes, index, index + 4), JsonNull));
        }
    }
    Err(JsonError::new(bytes, index))
}

#[inline(always)]
fn parse_true(bytes: &[u8], index: usize) -> Result<JsonItem, JsonError> {
    if index + 3 < bytes.len() {
        if bytes[index + 1] == b'r' && bytes[index + 2] == b'u' && bytes[index + 3] == b'e' {
            return Ok(JsonItem::new(Slice::from_bytes(bytes, index, index + 4), JsonTrue));
        }
    }
    Err(JsonError::new(bytes, index))
}

#[inline(always)]
fn parse_false(bytes: &[u8], index: usize) -> Result<JsonItem, JsonError> {
    if index + 4 < bytes.len() {
        if bytes[index + 1] == b'a' && bytes[index + 2] == b'l' && bytes[index + 3] == b's' && bytes[index + 4] == b'e' {
            return Ok(JsonItem::new(Slice::from_bytes(bytes, index, index + 5), JsonFalse));
        }
    }
    Err(JsonError::new(bytes, index))
}

#[inline(always)]
fn parse_number(bytes: &[u8], mut index: usize) -> Result<JsonItem, JsonError> {
    let mark = index;
    index += 1;
    while index < bytes.len() {
        match bytes[index] {
            b'0'..=b'9' | b'+' | b'-' | b'.' | b'e' | b'E' => {}
            _ => {
                return Ok(JsonItem::new(Slice::from_bytes(bytes, mark, index), JsonNumber));
            }
        }
        index += 1;
    }
    Err(JsonError::new(bytes, index))
}

#[inline(always)]
fn parse_string(bytes: &[u8], mut index: usize) -> Result<JsonItem, JsonError> {
    index += 1;
    let mark = index;
    let mut b = 0;
    while index < bytes.len() {
        let p = b;
        b = bytes[index];
        if b == b'"' {
            if p != b'\\' {
                return Ok(JsonItem::new(Slice::from_bytes(bytes, mark, index), JsonString));
            }
        }
        index += 1;
    }
    Err(JsonError::new(bytes, index))
}

#[inline(always)]
fn parse_item(bytes: &[u8], index: usize) -> Result<JsonItem, JsonError> {
    return match bytes[index] {
        b'n' => { Ok(parse_null(bytes, index)?) }
        b't' => { Ok(parse_true(bytes, index)?) }
        b'f' => { Ok(parse_false(bytes, index)?) }
        b'+' | b'-' | b'0'..=b'9' => { Ok(parse_number(bytes, index)?) }
        b'"' => { Ok(parse_string(bytes, index)?) }
        b'{' => { Ok(parse_map(bytes, index)?) }
        b'[' => { Ok(parse_array(bytes, index)?) }
        _ => {
            Err(JsonError::new(bytes, index))
        }
    };
}

#[inline(always)]
fn parse_map(bytes: &[u8], mut index: usize) -> Result<JsonItem, JsonError> {
    let mark = index;
    index += 1;
    let mut map = None;
    loop {
        // Spaces
        index = skip_spaces(bytes, index)?;

        // Check ending
        match bytes[index] {
            b'}' => {
                return Ok(JsonItem::new_map(Slice::from_bytes(bytes, mark, index + 1), map));
            }
            b',' => {
                index = skip_spaces(bytes, index + 1)?;
            }
            _ => {
                if !map.is_none() {
                    return Err(JsonError::new(bytes, index));
                }
            }
        }

        // Key
        let key = parse_string(bytes, index)?;
        index += shift_index(&key);

        // Separator
        index = skip_spaces(bytes, index)?;
        if bytes[index] != b':' {
            return Err(JsonError::new(bytes, index));
        } else {
            index = skip_spaces(bytes, index + 1)?;
        }

        // Value
        let item = parse_item(bytes, index)?;
        index += shift_index(&item);

        // Store
        if let Some(m) = &mut map {
            m.insert(Key::from(key.slice), item);
        } else {
            let mut m = BTreeMap::new();
            m.insert(Key::from(key.slice), item);
            map = Some(m);
        }
    }
}

#[inline(always)]
fn parse_array(bytes: &[u8], mut index: usize) -> Result<JsonItem, JsonError> {
    let mark = index;
    let mut array = None;
    index += 1;
    loop {
        // Spaces
        index = skip_spaces(bytes, index)?;

        // Check ending
        match bytes[index] {
            b']' => {
                return Ok(JsonItem::new_array(Slice::from_bytes(bytes, mark, index + 1), array));
            }
            b',' => {
                index = skip_spaces(bytes, index + 1)?;
            }
            _ => {
                if !array.is_none() {
                    return Err(JsonError::new(bytes, index));
                }
            }
        }

        // Item
        let item = parse_item(bytes, index)?;
        index += shift_index(&item);

        // Store
        if let Some(a) = &mut array {
            a.push(item);
        } else {
            let mut a = Vec::with_capacity(DEFAULT_VEC_CAPACITY);
            a.push(item);
            array = Some(a);
        }
    }
}

pub fn parse(source: &str) -> Result<JsonItem, JsonError> {
    let bytes = source.as_bytes();
    let mut index = 0_usize;
    index = skip_spaces(bytes, index)?;
    return match bytes[index] {
        b'{' => { parse_map(bytes, index) }
        b'[' => { parse_array(bytes, index) }
        _ => { Err(JsonError::new(bytes, index)) }
    };
}

#[cfg(test)]
mod tests {
    use crate::parse;

    const CORRECT_JSON: &str = " {\n\"test\": \"why not?\",\"b\": true,\"another\":  \"hey#çà@â&éè\" \r ,\"obj2\":{\"k\":{\"k2\":\"v\"}}, \"num\":4.2344, \"int\":-234,  \"obj\":{\"a\":\"b\", \"c\":\"d\"}, \"arr\":[1,2,3],\"bool\":false, \"exp\":3.3e-21, \"exp2\":-4.5e-213,\"exp3\":3.7391238e+24,\"depth\":[\"a\",[\"b\",\"c\"]]}  ";
    const INCORRECT_JSON: &str = "{\"test\": \"num\", \"int\":234[] ,,}";

    #[test]
    fn parse_correct() {
        match parse(CORRECT_JSON) {
            Ok(_) => {
                assert!(true);
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn parse_incorrect() {
        match parse(INCORRECT_JSON) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn parse_string() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                assert_eq!(parsed["test"].as_str(), Some("why not?"));
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn parse_float() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                assert_eq!(parsed["num"].as_f64(), Some(4.2344));
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn parse_int() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                assert_eq!(parsed["int"].as_i128(), Some(-234));
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn parse_object() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                assert_eq!(parsed["obj"]["a"].as_str(), Some("b"));
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn parse_object_depth() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                assert_eq!(parsed["obj2"]["k"]["k2"].as_str(), Some("v"));
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn traverse_object() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                let mut iterator = parsed["obj"].entries().unwrap();
                let (k, v) = iterator.next().unwrap();
                assert_eq!(k.as_str(), "a");
                assert_eq!(v.as_str(), Some("b"));
                let (k, v) = iterator.next().unwrap();
                assert_eq!(k.as_str(), "c");
                assert_eq!(v.as_str(), Some("d"));
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn parse_array() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                assert_eq!(parsed["arr"][1].as_i128(), Some(2));
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn parse_array_depth() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                assert_eq!(parsed["depth"][1][1].as_str(), Some("c"));
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn traverse_array() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                let mut iterator = parsed["arr"].elements().unwrap();
                assert_eq!(iterator.next().unwrap().as_i128(), Some(1));
                assert_eq!(iterator.next().unwrap().as_i128(), Some(2));
                assert_eq!(iterator.next().unwrap().as_i128(), Some(3));
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn parse_bool() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                assert_eq!(parsed["bool"].as_bool(), Some(false));
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn parse_exp() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                match parsed["exp"].as_f64() {
                    None => { assert!(false); }
                    Some(value) => { assert!(f64::abs(value / 3.3e-21 - 1.0) < 1e-8); }   // floating point error
                }
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn parse_exp_3_digits() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                match parsed["exp2"].as_f64() {
                    None => { assert!(false); }
                    Some(value) => { assert!(f64::abs(value / -4.5e-213 - 1.0) < 1e-8); }   // floating point error
                }
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn missing_key() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                assert_eq!(parsed["a"].exists(), false);
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }

    #[test]
    fn missing_key_get_value() {
        match parse(CORRECT_JSON) {
            Ok(parsed) => {
                match parsed["a"][1].as_i128() {
                    None => { assert!(true); }
                    Some(_) => { assert!(false); }
                }
            }
            Err(error) => {
                assert!(false, "{}", error.to_string());
            }
        }
    }
}