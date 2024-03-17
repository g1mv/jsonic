use std::fs::read_to_string;

use divan::Bencher;
use serde_json::Value;

fn main() {
    divan::main();
}

fn parse_file(bencher: Bencher, path: &str) {
    let in_memory_json = read_to_string(path).unwrap();
    assert!(sonic_rs::from_str::<Value>(&in_memory_json).is_ok());

    bencher.bench_local(move || {
        sonic_rs::from_str::<Value>(&in_memory_json)
    });
}

#[divan::bench(args = ["./benches/data/canada.json", "./benches/data/citm_catalog.json", "./benches/data/twitter.json"])]
fn parse(bencher: Bencher, path: &str) {
    parse_file(bencher, path);
}