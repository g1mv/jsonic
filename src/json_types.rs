#[derive(Debug, PartialEq)]
pub enum JsonType {
    JsonTypeNull,
    JsonTypeBoolean,
    JsonTypeString,
    JsonTypeNumber,
    JsonTypeObject,
    JsonTypeArray,
    JsonTypeEmpty
}

#[derive(Debug, PartialEq)]
pub enum JsonNumberType {
    JsonNumberTypeFloat,
    JsonNumberTypeInteger,
}