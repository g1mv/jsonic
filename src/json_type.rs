#[derive(PartialEq, Debug)]
pub enum JsonType {
    JsonNull,
    JsonTrue,
    JsonFalse,
    JsonString,
    JsonNumber,
    JsonEmptyMap,
    JsonMap,
    JsonEmptyArray,
    JsonArray,
    Void,
}