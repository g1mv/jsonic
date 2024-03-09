use std::fs::read_to_string;

use divan::Bencher;

fn main() {
    divan::main();
}

fn parse_file(bencher: Bencher, path: &str) {
    let in_memory_json = read_to_string(path).unwrap();

    bencher.bench_local(move || {
        json::parse(&in_memory_json)
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