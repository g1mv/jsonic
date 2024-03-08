use std::fs::read_to_string;

use divan::Bencher;

use jsonic::json_parser::JsonParser;

fn main() {
    divan::main();
}

fn parse_file(bencher: Bencher, path: &str) {
    let in_memory_json = match read_to_string(path) {
        Ok(text) => { text }
        Err(_) => { return; }
    };

    bencher.bench_local(move || {
        match JsonParser::new(&in_memory_json).parse() {
            Ok(_) => {}
            Err(_) => {}
        }
    });
}

#[divan::bench]
fn parse_canada(bencher: Bencher) {
    parse_file(bencher, "./benches/data/canada.json");
}

#[divan::bench]
fn parse_citm_catalog(bencher: Bencher) {
    parse_file(bencher, "./benches/data/citm_catalog.json");
}

#[divan::bench]
fn parse_twitter(bencher: Bencher) {
    parse_file(bencher, "./benches/data/twitter.json");
}