use std::collections::BTreeMap;

use crate::json_error::JsonError;
use crate::json_item::JsonItem;
use crate::json_type::JsonType::{False, Null, Number, String, True};
use crate::slice::Slice;

pub mod json_error;
pub mod slice;
pub mod json_item;

pub mod json_type;

fn parse_null(source: &str, index: usize) -> Result<JsonItem, JsonError> {
    let bytes = source.as_bytes();
    if index + 3 < source.len() {
        if bytes[index + 1] == b'u' && bytes[index + 2] == b'l' && bytes[index + 3] == b'l' {
            return Ok(JsonItem::new(Slice::new(source, index, index + 4), Null));
        }
    }
    Err(JsonError::new(source, index))
}

fn parse_true(source: &str, index: usize) -> Result<JsonItem, JsonError> {
    let bytes = source.as_bytes();
    if index + 3 < source.len() {
        if bytes[index + 1] == b'r' && bytes[index + 2] == b'u' && bytes[index + 3] == b'e' {
            return Ok(JsonItem::new(Slice::new(source, index, index + 4), True));
        }
    }
    Err(JsonError::new(source, index))
}

fn parse_false(source: &str, index: usize) -> Result<JsonItem, JsonError> {
    let bytes = source.as_bytes();
    if index + 4 < source.len() {
        if bytes[index + 1] == b'a' && bytes[index + 2] == b'l' && bytes[index + 3] == b's' && bytes[index + 4] == b'e' {
            return Ok(JsonItem::new(Slice::new(source, index, index + 5), False));
        }
    }
    Err(JsonError::new(source, index))
}

fn parse_number(source: &str, mut index: usize) -> Result<JsonItem, JsonError> {
    let bytes = source.as_bytes();
    let mark = index;
    index += 1;
    while index < source.len() {
        let b = bytes[index];
        match b {
            b'0'..=b'9' | b'+' | b'-' | b'.' | b'e' | b'E' => {}
            _ => {
                return Ok(JsonItem::new(Slice::new(source, mark, index), Number));
            }
        }
        index += 1;
    }
    Err(JsonError::new(source, index))
}

fn parse_string(source: &str, mut index: usize) -> Result<JsonItem, JsonError> {
    let bytes = source.as_bytes();
    index += 1;
    let mark = index;
    let mut b = 0;
    while index < source.len() {
        let p = b;
        b = bytes[index];
        if b == b'"' {
            if p != b'\\' {
                return Ok(JsonItem::new(Slice::new(source, mark, index), String));
            }
        }
        index += 1;
    }
    Err(JsonError::new(source, index))
}

fn parse_map(source: &str, mut index: usize) -> Result<JsonItem, JsonError> {
    let bytes = source.as_bytes();
    let mark = index;
    index += 1;
    let mut map = BTreeMap::new();
    let mut key = None;
    'main: loop {
        return if let Some(k) = key {
            while index < source.len() {
                let b = bytes[index];
                match b {
                    b' ' | b':' => {}
                    b'n' => {
                        let json_null = parse_null(source, index)?;
                        index = json_null.slice.end;
                        map.insert(k, json_null);
                        key = None;
                        continue 'main;
                    }
                    b't' => {
                        let json_true = parse_true(source, index)?;
                        index = json_true.slice.end;
                        map.insert(k, json_true);
                        key = None;
                        continue 'main;
                    }
                    b'f' => {
                        let json_false = parse_false(source, index)?;
                        index = json_false.slice.end;
                        map.insert(k, json_false);
                        key = None;
                        continue 'main;
                    }
                    b'+' | b'-' | b'0'..=b'9' => {
                        let json_number = parse_number(source, index)?;
                        index = json_number.slice.end;
                        map.insert(k, json_number);
                        key = None;
                        continue 'main;
                    }
                    b'"' => {
                        let json_string = parse_string(source, index)?;
                        index = json_string.slice.end + 1;
                        map.insert(k, json_string);
                        key = None;
                        continue 'main;
                    }
                    b'{' => {
                        let json_map = parse_map(source, index)?;
                        index = json_map.slice.end;
                        map.insert(k, json_map);
                        key = None;
                        continue 'main;
                    }
                    b'[' => {
                        let json_array = parse_array(source, index)?;
                        index = json_array.slice.end;
                        map.insert(k, json_array);
                        key = None;
                        continue 'main;
                    }
                    _ => {
                        return Err(JsonError::new(source, index));
                    }
                }
                index += 1;
            }
            Err(JsonError::new(source, index))
        } else {
            while index < source.len() {
                let b = bytes[index];
                match b {
                    b' ' | b',' | b'\n' | b'\r' | b'\t' => {}
                    b'"' => {
                        let json_string = parse_string(source, index)?;
                        index = json_string.slice.end + 1;
                        key = Some(json_string.slice.as_str().unwrap().to_owned());
                        continue 'main;
                    }
                    b'}' => {
                        return Ok(JsonItem::new_map(Slice::new(source, mark, index + 1), map));
                    }
                    _ => {
                        return Err(JsonError::new(source, index));
                    }
                }
                index += 1;
            }
            Err(JsonError::new(source, index))
        };
    }
}

fn parse_array(source: &str, mut index: usize) -> Result<JsonItem, JsonError> {
    let bytes = source.as_bytes();
    let mark = index;
    let mut array = Vec::new();
    index += 1;
    while index < source.len() {
        let b = bytes[index];
        match b {
            b' ' | b',' | b'\n' | b'\r' | b'\t' => {}
            b'n' => {
                let json_null = parse_null(source, index)?;
                index = json_null.slice.end;
                array.push(json_null);
                continue;
            }
            b't' => {
                let json_true = parse_true(source, index)?;
                index = json_true.slice.end;
                array.push(json_true);
                continue;
            }
            b'f' => {
                let json_false = parse_false(source, index)?;
                index = json_false.slice.end;
                array.push(json_false);
                continue;
            }
            b'+' | b'-' | b'0'..=b'9' => {
                let json_number = parse_number(source, index)?;
                index = json_number.slice.end;
                array.push(json_number);
                continue;
            }
            b'"' => {
                let json_string = parse_string(source, index)?;
                index = json_string.slice.end + 1;
                array.push(json_string);
                continue;
            }
            b'{' => {
                let json_map = parse_map(source, index)?;
                index = json_map.slice.end;
                array.push(json_map);
                continue;
            }
            b'[' => {
                let json_array = parse_array(source, index)?;
                index = json_array.slice.end;
                array.push(json_array);
                continue;
            }
            b']' => {
                return Ok(JsonItem::new_array(Slice::new(source, mark, index + 1), array));
            }
            _ => {
                return Err(JsonError::new(source, index));
            }
        }
        index += 1;
    }
    Err(JsonError::new(source, index))
}

pub fn parse(source: &str) -> Result<JsonItem, JsonError> {
    let bytes = source.as_bytes();
    let mut index = 0_usize;
    while index < source.len() {
        match bytes[index] {
            b' ' | b'\n' | b'\r' | b'\t' => {}
            b'{' => {
                return parse_map(source, index);
            }
            b'[' => {
                return parse_array(source, index);
            }
            _ => {
                return Err(JsonError::new(source, index));
            }
        }
        index += 1;
    }
    Err(JsonError::new(source, index))
}

#[cfg(test)]
mod tests {
    use crate::parse;

    const CORRECT_JSON: &str = " {\n\"test\": \"why not?\",\"b\": true,\"another\":  \"hey#çà@â&éè\" \r ,\"obj2\":{\"k\":{\"k2\":\"v\"}} \"num\":4.2344, \"int\":-234,  \"obj\":{\"a\":\"b\", \"c\":\"d\"}, \"arr\":[1,2,3],\"bool\":false, \"exp\":3.3e-21, \"exp2\":-4.5e-213,\"exp3\":3.7391238e+24,\"depth\":[\"a\",[\"b\",\"c\"]]}  ";
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
                assert_eq!(k, "a");
                assert_eq!(v.as_str(), Some("b"));
                let (k, v) = iterator.next().unwrap();
                assert_eq!(k, "c");
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