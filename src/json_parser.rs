use std::ops::{AddAssign, Neg, Shl};

use crate::json_element::JsonElement;
use crate::json_elements::json_array::JsonArray;
use crate::json_elements::json_number::JsonNumber;
use crate::json_elements::json_object::JsonObject;
use crate::json_error::JsonError;
use crate::json_types::JsonNumberType;
use crate::slice::Slice;

const EXP_MATRIX: [[f64; 65]; 2] = [[1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16, 1e17, 1e18, 1e19, 1e20, 1e21, 1e22, 1e23, 1e24, 1e25, 1e26, 1e27, 1e28, 1e29, 1e30, 1e31, 1e32, 1e33, 1e34, 1e35, 1e36, 1e37, 1e38, 1e39, 1e40, 1e41, 1e42, 1e43, 1e44, 1e45, 1e46, 1e47, 1e48, 1e49, 1e50, 1e51, 1e52, 1e53, 1e54, 1e55, 1e56, 1e57, 1e58, 1e59, 1e60, 1e61, 1e62, 1e63, 1e64], [1e-0, 1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10, 1e-11, 1e-12, 1e-13, 1e-14, 1e-15, 1e-16, 1e-17, 1e-18, 1e-19, 1e-20, 1e-21, 1e-22, 1e-23, 1e-24, 1e-25, 1e-26, 1e-27, 1e-28, 1e-29, 1e-30, 1e-31, 1e-32, 1e-33, 1e-34, 1e-35, 1e-36, 1e-37, 1e-38, 1e-39, 1e-40, 1e-41, 1e-42, 1e-43, 1e-44, 1e-45, 1e-46, 1e-47, 1e-48, 1e-49, 1e-50, 1e-51, 1e-52, 1e-53, 1e-54, 1e-55, 1e-56, 1e-57, 1e-58, 1e-59, 1e-60, 1e-61, 1e-62, 1e-63, 1e-64]];

pub struct JsonParser<'a> {
    source: &'a [u8],
    index: usize,
    length: usize,
}

impl<'a> JsonParser<'a> {
    pub fn new(text: &'a str) -> JsonParser<'a> {
        let source = text.as_bytes();
        JsonParser {
            source,
            index: 0,
            length: source.len(),
        }
    }

    fn read_offset_byte(&self, offset: usize) -> u8 {
        return self.source[offset + self.index];
    }

    fn next_byte(&self) -> u8 {
        return self.source[self.index];
    }

    // fn next_2_bytes(&self) -> usize {
    //     return (self.next_byte() as usize).shl(8) + self.read_offset_byte(1) as usize;
    // }

    fn next_3_bytes(&self) -> usize {
        return (self.next_byte() as usize).shl(16) + (self.read_offset_byte(1) as usize).shl(8) + self.read_offset_byte(2) as usize;
    }

    fn next_4_bytes(&self) -> usize {
        return (self.next_byte() as usize).shl(24) + (self.read_offset_byte(1) as usize).shl(16) + (self.read_offset_byte(2) as usize).shl(8) + self.read_offset_byte(3) as usize;
    }

    fn increment_index(&mut self) {
        self.index.add_assign(1);
    }

    fn parse_null(&mut self) -> Result<JsonElement<'a>, JsonError> {
        let mark = self.index;
        self.increment_index();
        if self.index + 2 < self.length {
            return if self.next_3_bytes() == ((b'u' as usize) << 16) + ((b'l' as usize) << 8) + b'l' as usize {
                self.index += 3;
                Ok(JsonElement::from_null(Slice::new(self.source, mark, self.index)))
            } else {
                Err(JsonError::new(self.source, self.index))
            };
        }
        return Err(JsonError::new(self.source, self.index));
    }

    fn parse_true(&mut self) -> Result<JsonElement<'a>, JsonError> {
        let mark = self.index;
        self.increment_index();
        if self.index + 2 < self.length {
            return if self.next_3_bytes() == ((b'r' as usize) << 16) + ((b'u' as usize) << 8) + b'e' as usize {
                self.index += 3;
                Ok(JsonElement::from_boolean(true, Slice::new(self.source, mark, self.index)))
            } else {
                Err(JsonError::new(self.source, self.index))
            };
        }
        return Err(JsonError::new(self.source, self.index));
    }

    fn parse_false(&mut self) -> Result<JsonElement<'a>, JsonError> {
        let mark = self.index;
        self.increment_index();
        if self.index + 3 < self.length {
            return if self.next_4_bytes() == ((b'a' as usize) << 24) + ((b'l' as usize) << 16) + ((b's' as usize) << 8) + b'e' as usize {
                self.index += 4;
                Ok(JsonElement::from_boolean(false, Slice::new(self.source, mark, self.index)))
            } else {
                Err(JsonError::new(self.source, self.index))
            };
        }
        return Err(JsonError::new(self.source, self.index));
    }

    fn parse_exponent(&mut self) -> Result<f64, JsonError> {
        self.increment_index();
        let mut number = 0_i32;
        let mut sign = 1_i32;
        while self.index < self.length {
            let b = self.next_byte();
            match b {
                b'-' => {
                    sign = -1_i32;
                }
                b'0'..=b'9' => {
                    match number.overflowing_mul(10_i32) {
                        (_, true) => {
                            return Err(JsonError::new(self.source, self.index));
                        }
                        (result, false) => {
                            number = result + (b - b'0') as i32;
                        }
                    }
                }
                // b' ' | b',' | b'}' | b']' => {
                //     return Ok(10.0_f64.powi(sign * number));
                // }
                _ => {
                    return if number < EXP_MATRIX[0].len() as i32 {
                        Ok(EXP_MATRIX[if sign == 1 { 0_usize } else { 1_usize }][number as usize])
                    } else {
                        Ok(10.0_f64.powi(sign * number))
                    };
                }
            }
            self.increment_index();
        }
        return Err(JsonError::new(self.source, self.index));
    }

    fn parse_digits(&mut self) -> Result<i128, JsonError> {
        let mut number = 0_i128;
        // while self.index + 3 < self.length {
        //     let next_4 = self.next_4_bytes();
        //     if count_between(next_4, b'0' as u64, b'9' as u64) == 4 {
        //         num = num * 10000_i64
        //             + 1000_i64 * (unsafe { *self.byte_ptr.offset(self.index as isize) } - b'0') as i64
        //             + 100_i64 * (unsafe { *self.byte_ptr.offset(1 + self.index as isize) } - b'0') as i64
        //             + 10_i64 * (unsafe { *self.byte_ptr.offset(2 + self.index as isize) } - b'0') as i64
        //             + (unsafe { *self.byte_ptr.offset(3 + self.index as isize) } - b'0') as i64;
        //         self.index += 4;
        //         continue;
        //     } else {
        //         break;
        //     }
        // }

        while self.index < self.length {
            let b = self.next_byte();
            match b {
                b'0'..=b'9' => {
                    match number.overflowing_mul(10_i128) {
                        (_, true) => {
                            return Err(JsonError::new(self.source, self.index));
                        }
                        (result, false) => {
                            number = result + (b - b'0') as i128;
                        }
                    }
                }
                _ => {
                    return Ok(number);
                }
            }
            self.increment_index();
        }
        return Err(JsonError::new(self.source, self.index));
    }

    fn parse_number(&mut self) -> Result<JsonElement<'a>, JsonError> {
        let mark = self.index;
        let mut after_dot = 0_i128;
        let mut dot_multiplier = 0.0_f64;
        let mut neg = false;

        // Check if sign is present
        if self.next_byte() == b'-' {
            neg = true;
            self.increment_index();
        }

        // Parse continuous digits
        let mut before_dot = self.parse_digits()?;

        // Parse eventual remainder
        while self.index < self.length {
            let b = self.next_byte();
            match b {
                b'.' => {
                    self.increment_index();
                    let dot_mark = self.index;
                    after_dot = self.parse_digits()?;
                    dot_multiplier = EXP_MATRIX[1][self.index - dot_mark];
                    continue;
                }
                b'e' | b'E' => {
                    let exponent = self.parse_exponent()?;
                    if neg {
                        before_dot = before_dot.neg();
                        after_dot = after_dot.neg();
                    }
                    return Ok(JsonElement::from_number(JsonNumber { num_i128: before_dot, num_f64: exponent * (before_dot as f64 + after_dot as f64 * dot_multiplier), detected_type: JsonNumberType::JsonNumberTypeFloat },
                                                       Slice::new(self.source, mark, self.index)));
                }
                _ => {
                    if neg {
                        before_dot = before_dot.neg();
                        after_dot = after_dot.neg();
                    }
                    return Ok(JsonElement::from_number(JsonNumber { num_i128: before_dot, num_f64: before_dot as f64 + after_dot as f64 * dot_multiplier, detected_type: if dot_multiplier != 0.0 { JsonNumberType::JsonNumberTypeFloat } else { JsonNumberType::JsonNumberTypeInteger } },
                                                       Slice::new(self.source, mark, self.index)));
                }
            }
        }
        return Err(JsonError::new(self.source, self.index));
    }

    fn parse_string(&mut self) -> Result<JsonElement<'a>, JsonError> {
        self.increment_index();
        let mark = self.index;
        let mut b = 0;
        while self.index < self.length {
            let p = b;
            b = self.next_byte();
            match b {
                b'"' => {
                    if p != b'\\' {
                        self.increment_index();
                        return Ok(JsonElement::from_string(Slice::new(self.source, mark, self.index - 1)));
                    }
                }
                _ => {}
            }
            self.increment_index();
        }
        return Err(JsonError::new(self.source, self.index));
    }

    fn parse_array(&mut self) -> Result<JsonElement<'a>, JsonError> {
        let mark = self.index;
        self.increment_index();
        let mut array = JsonArray::new();
        while self.index < self.length {
            let b = self.next_byte();
            match b {
                b' ' | b',' | b'\n' | b'\r' => {}
                b'n' => {
                    array.vec.push(self.parse_null()?);
                    continue;
                }
                b't' => {
                    array.vec.push(self.parse_true()?);
                    continue;
                }
                b'f' => {
                    array.vec.push(self.parse_false()?);
                    continue;
                }
                b'-' | b'0'..=b'9' => {
                    array.vec.push(self.parse_number()?);
                    continue;
                }
                b'"' => {
                    array.vec.push(self.parse_string()?);
                    continue;
                }
                b'{' => {
                    array.vec.push(self.parse_object()?);
                    continue;
                }
                b'[' => {
                    // self.parse_array()?.vec.iter().map(|v| array.push(v));
                    array.vec.push(self.parse_array()?);
                    continue;
                }
                b']' => {
                    self.increment_index();
                    return Ok(JsonElement::from_array(array, Slice::new(self.source, mark, self.index)));
                }
                _ => {
                    return Err(JsonError::new(self.source, self.index));
                }
            }
            self.increment_index();
        }
        return Err(JsonError::new(self.source, self.index));
    }

    fn parse_object(&mut self) -> Result<JsonElement<'a>, JsonError> {
        let mark = self.index;
        self.increment_index();
        let mut object = JsonObject::new();
        let mut key = None;
        while self.index < self.length {
            let b = self.next_byte();
            match key {
                None => {
                    match b {
                        b' ' | b',' | b'\n' | b'\r' => {}
                        b'"' => {
                            let slice = self.parse_string()?.slice;
                            key = Some(slice.as_str().to_owned());
                        }
                        b'}' => {
                            self.increment_index();
                            return Ok(JsonElement::from_object(object, Slice::new(self.source, mark, self.index)));
                        }
                        _ => {
                            return Err(JsonError::new(self.source, self.index));
                        }
                    }
                }
                Some(_) => {
                    match b {
                        b' ' | b':' => {}
                        b'n' => {
                            object.map.insert(key.unwrap(), self.parse_null()?);
                            key = None;
                            continue;
                        }
                        b't' => {
                            object.map.insert(key.unwrap(), self.parse_true()?);
                            key = None;
                            continue;
                        }
                        b'f' => {
                            object.map.insert(key.unwrap(), self.parse_false()?);
                            key = None;
                            continue;
                        }
                        b'-' | b'0'..=b'9' => {
                            object.map.insert(key.unwrap(), self.parse_number()?);
                            key = None;
                            continue;
                        }
                        b'"' => {
                            object.map.insert(key.unwrap(), self.parse_string()?);
                            key = None;
                            continue;
                        }
                        b'{' => {
                            object.map.insert(key.unwrap(), self.parse_object()?);
                            key = None;
                            continue;
                        }
                        b'[' => {
                            object.map.insert(key.unwrap(), self.parse_array()?);
                            key = None;
                            continue;
                        }
                        _ => {
                            return Err(JsonError::new(self.source, self.index));
                        }
                    }
                }
            }
            self.increment_index();
        }
        return Err(JsonError::new(self.source, self.index));
    }

    pub fn parse(&mut self) -> Result<JsonElement, JsonError> {
        while self.index < self.length {
            let b = self.next_byte();
            match b {
                b' ' | b'\n' | b'\r' => {}
                b'{' => {
                    return Ok(self.parse_object()?);
                }
                b'[' => {
                    return Ok(self.parse_array()?);
                }
                _ => {
                    return Err(JsonError::new(self.source, self.index));
                }
            }
            self.increment_index();
        }
        return Err(JsonError::new(self.source, self.index));
    }
}