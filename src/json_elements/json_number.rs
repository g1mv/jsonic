use crate::json_types::JsonNumberType;

#[derive(Debug)]
pub struct JsonNumber {
    pub num_i64: i64,
    pub num_f64: f64,
    pub detected_type: JsonNumberType,
}

impl JsonNumber {
    pub const fn empty() -> Self {
        return JsonNumber { num_i64: 0, num_f64: 0.0, detected_type: JsonNumberType::JsonNumberTypeInteger };
    }

    pub fn i64(&self) -> i64 {
        return self.num_i64;
    }

    pub fn f64(&self) -> f64 {
        return self.num_f64;
    }
}