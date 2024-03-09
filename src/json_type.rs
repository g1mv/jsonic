#[derive(PartialEq, Debug)]
pub enum JsonType {
    Null,
    True,
    False,
    String,
    Number,
    Map,
    Array,
    Empty,
}