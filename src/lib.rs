pub mod json_parser;
pub mod json_element;
pub mod json_elements;
pub mod json_error;
pub mod json_types;
pub mod slice;

#[cfg(test)]
mod tests {
    use crate::json_parser::JsonParser;

    const CORRECT_JSON: &str = " {\n\"test\": \"why not?\",\"another\":  \"hey#çà@â&éè\" \r ,\"obj2\":{\"k\":{\"k2\":\"v\"}} \"num\":4.2344, \"int\":-234,  \"obj\":{\"a\":\"b\", \"c\":\"d\"}, \"arr\":[1,2,3],\"bool\":false, \"exp\":3.3e-21, \"exp2\":-4.5e-213,\"exp3\":3.7391238e+24,\"depth\":[\"a\",[\"b\",\"c\"]]}  ";
    const INCORRECT_JSON: &str = "{\"test\": \"num\", \"int\":234[] ,,}";

    #[test]
    fn parse_correct() {
        match JsonParser::new(CORRECT_JSON).parse() {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_incorrect() {
        match JsonParser::new(INCORRECT_JSON).parse() {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn parse_string() {
        match JsonParser::new(CORRECT_JSON).parse() {
            Ok(parsed) => {
                assert_eq!(parsed["test"].as_str(), Some("why not?"));
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_float() {
        match JsonParser::new(CORRECT_JSON).parse() {
            Ok(parsed) => {
                assert_eq!(parsed["num"].as_f64(), Some(4.2344));
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_int() {
        match JsonParser::new(CORRECT_JSON).parse() {
            Ok(parsed) => {
                assert_eq!(parsed["int"].as_i128(), Some(-234));
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_object() {
        match JsonParser::new(CORRECT_JSON).parse() {
            Ok(parsed) => {
                assert_eq!(parsed["obj"]["a"].as_str(), Some("b"));
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_object_depth() {
        match JsonParser::new(CORRECT_JSON).parse() {
            Ok(parsed) => {
                assert_eq!(parsed["obj2"]["k"]["k2"].as_str(), Some("v"));
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn traverse_object() {
        match JsonParser::new(CORRECT_JSON).parse() {
            Ok(parsed) => {
                let mut iterator = parsed["obj"].entries().unwrap();
                let a = iterator.next().unwrap();
                assert_eq!(a.0, "a");
                assert_eq!(a.1.as_str(), Some("b"));
                let b = iterator.next().unwrap();
                assert_eq!(b.0, "c");
                assert_eq!(b.1.as_str(), Some("d"));
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_array() {
        match JsonParser::new(CORRECT_JSON).parse() {
            Ok(parsed) => {
                assert_eq!(parsed["arr"][1].as_i128(), Some(2));
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_array_depth() {
        match JsonParser::new(CORRECT_JSON).parse() {
            Ok(parsed) => {
                assert_eq!(parsed["depth"][1][1].as_str(), Some("c"));
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn traverse_array() {
        match JsonParser::new(CORRECT_JSON).parse() {
            Ok(parsed) => {
                let mut iterator = parsed["arr"].elements().unwrap();
                assert_eq!(iterator.next().unwrap().as_i128(), Some(1));
                assert_eq!(iterator.next().unwrap().as_i128(), Some(2));
                assert_eq!(iterator.next().unwrap().as_i128(), Some(3));
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_bool() {
        match JsonParser::new(CORRECT_JSON).parse() {
            Ok(parsed) => {
                assert_eq!(parsed["bool"].as_bool(), Some(false));
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_exp() {
        match JsonParser::new(CORRECT_JSON).parse() {
            Ok(parsed) => {
                match parsed["exp"].as_f64() {
                    None => { assert!(false); }
                    Some(value) => { assert!(f64::abs(value / 3.3e-21 - 1.0) < 1e-8); }   // floating point error
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_exp_3_digits() {
        match JsonParser::new(CORRECT_JSON).parse() {
            Ok(parsed) => {
                match parsed["exp2"].as_f64() {
                    None => { assert!(false); }
                    Some(value) => { assert!(f64::abs(value / -4.5e-213 - 1.0) < 1e-8); }   // floating point error
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn missing_key() {
        match JsonParser::new(CORRECT_JSON).parse() {
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
        match JsonParser::new(CORRECT_JSON).parse() {
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
}