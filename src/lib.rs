use crate::json_parser::JsonParser;

pub mod json_parser;
pub mod json_element;
pub mod json_elements;
pub mod json_error;
pub mod json_types;
pub mod slice;

#[test]
fn parse() {
    let text = "{\"test\": \"why not?\",\"another\":\"hey\"  ,\"num\":4.2344}";
    let mut parser = JsonParser::new(text);

    match parser.parse() {
        Ok(parsed) => {
            match parsed["test"].as_str() {
                None => {
                    assert!(false);
                }
                Some(value) => {
                    assert_eq!(value, "why not?");
                }
            }
        }
        Err(_) => {
            assert!(false);
        }
    }
}