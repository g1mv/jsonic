use std::fs::read_to_string;

use divan::Bencher;
use serde_json::Value;

fn main() {
    divan::main();
}

fn parse_file(bencher: Bencher, path: &str) {
    let in_memory_json = read_to_string(path).unwrap();

    let mut copy = in_memory_json.to_owned();
    assert!(unsafe { simd_json::from_str::<Value>(&mut copy).is_ok() });

    bencher.with_inputs(|| {
        in_memory_json.to_owned()
    }).bench_local_values(move |mut copy| unsafe {
        simd_json::from_str::<Value>(&mut copy)
    });
}

#[divan::bench(args = ["./benches/data/canada.json", "./benches/data/citm_catalog.json", "./benches/data/twitter.json"])]
fn parse(bencher: Bencher, path: &str) {
    parse_file(bencher, path);
}