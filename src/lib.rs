pub mod json_parser;
pub mod json_element;
pub mod json_elements;
pub mod json_error;
pub mod json_types;
pub mod slice;

#[cfg(test)]
mod tests {
    use crate::json_parser::JsonParser;

    const CORRECT_JSON: &str = "{\"test\": \"why not?\",\"another\":  \"hey#çà@â&éè\"  , \"num\":4.2344, \"int\":234,  \"obj\":{\"a\":\"b\"}, \"arr\":[1,2,3],\"bool\":false}";
    const INCORRECT_JSON: &str = "{\"test\": \"num\", \"int\":234[] ,,}";

    #[test]
    fn parse_string() {
        let mut parser = JsonParser::new(CORRECT_JSON);

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

    #[test]
    fn parse_float() {
        let mut parser = JsonParser::new(CORRECT_JSON);

        match parser.parse() {
            Ok(parsed) => {
                match parsed["num"].as_f64() {
                    None => { assert!(false); }
                    Some(value) => { assert_eq!(value, 4.2344); }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_int() {
        let mut parser = JsonParser::new(CORRECT_JSON);

        match parser.parse() {
            Ok(parsed) => {
                match parsed["int"].as_i128() {
                    None => { assert!(false); }
                    Some(value) => { assert_eq!(value, 234); }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_obj() {
        let mut parser = JsonParser::new(CORRECT_JSON);

        match parser.parse() {
            Ok(parsed) => {
                match parsed["obj"]["a"].as_str() {
                    None => { assert!(false); }
                    Some(value) => { assert_eq!(value, "b"); }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_array() {
        let mut parser = JsonParser::new(CORRECT_JSON);

        match parser.parse() {
            Ok(parsed) => {
                match parsed["arr"][1].as_i128() {
                    None => { assert!(false); }
                    Some(value) => { assert_eq!(value, 2); }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_bool() {
        let mut parser = JsonParser::new(CORRECT_JSON);

        match parser.parse() {
            Ok(parsed) => {
                match parsed["bool"].as_bool() {
                    None => { assert!(false); }
                    Some(value) => { assert_eq!(value, false); }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn missing_key() {
        let mut parser = JsonParser::new(CORRECT_JSON);

        match parser.parse() {
            Ok(parsed) => {
                assert_eq!(parsed["a"].exists(), false);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn missing_key_get_value() {
        let mut parser = JsonParser::new(CORRECT_JSON);

        match parser.parse() {
            Ok(parsed) => {
                match parsed["a"][1].as_i128() {
                    None => { assert!(true); }
                    Some(_) => { assert!(false); }
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn incorrect_json() {
        let mut parser = JsonParser::new(INCORRECT_JSON);

        match parser.parse() {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }
}