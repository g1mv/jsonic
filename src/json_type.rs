/// An enum representing JSON types
/// * `JsonNull` &rarr; null
/// * `JsonTrue`, `JsonFalse` &rarr; true, false
/// * `JsonString` &rarr; a string
/// * `JsonNumber` &rarr; an integer or float
/// * `JsonMap` &rarr; a JSON object
/// * `JsonArray` &rarr; a JSON array
/// * `Empty` &rarr; Element not found. See code below.
///
/// ```rust
/// use jsonic::json_item::JsonItem;
/// use jsonic::json_type::JsonType::Empty;
///
/// let json = "{\"a\":\"b\"}";
///
/// if let Ok(parsed) = jsonic::parse(json) {
///     assert!(parsed["c"].get_type() == &Empty);
/// }
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