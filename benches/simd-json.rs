#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use divan::Bencher;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use serde_json::Value;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use std::fs::read_to_string;

fn main() {
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    divan::main();
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
fn parse_file(bencher: Bencher, path: &str) {
    let in_memory_json = read_to_string(path).unwrap();

    let mut copy = in_memory_json.to_owned();
    assert!(unsafe { simd_json::from_str::<Value>(&mut copy).is_ok() });

    bencher
        .with_inputs(|| in_memory_json.to_owned())
        .bench_local_values(move |mut copy| unsafe { simd_json::from_str::<Value>(&mut copy) });
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[divan::bench(args = ["./benches/data/canada.json", "./benches/data/citm_catalog.json", "./benches/data/twitter.json"])]
fn parse(bencher: Bencher, path: &str) {
    parse_file(bencher, path);
}
