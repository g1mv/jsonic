mod parser;
mod json_element;
mod json_elements;
mod json_error;
mod json_types;
mod slice;

use crate::json_element::JsonElement;
use crate::json_error::JsonError;
use crate::parser::Parser;

fn main() {
    let mut parser = Parser::new("{\"test\":\"why not?\",\"another\":\"hey\"}");
    match parser.parse() {
        Ok(parsed) => {
            println!("{:?}", parsed);
        }
        Err(error) => {
            eprintln!("{:?}", error);
        }
    }
}