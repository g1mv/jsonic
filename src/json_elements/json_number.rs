use crate::json_types::JsonNumberType;

#[derive(Debug)]
pub struct JsonNumber {
    pub num_i128: i128,
    pub num_f64: f64,
    pub detected_type: JsonNumberType,
}

impl JsonNumber {
    pub fn i128(&self) -> i128 {
        return self.num_i128;
    }

    pub fn f64(&self) -> f64 {
        return self.num_f64;
    }
}