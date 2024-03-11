#[derive(PartialEq, Debug)]
pub enum JsonType {
    JsonNull,
    JsonTrue,
    JsonFalse,
    JsonString,
    JsonNumber,
    JsonMap,
    JsonArray,
    Empty,
}