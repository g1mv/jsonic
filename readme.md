# jsonic

Fast, small JSON parsing library for rust with no dependencies

##

### Quick overview
**jsonic** is a JSON parser. It aims at providing high-speed extraction of JSON data.
It does not convert JSON to structs at this stage.

### Performance
Here are some of the design choices for this library:

* small-footprint data structures to speedup memory/cache access
* object containers with hybrid data structures, using arrays to store low numbers of key/value pairs, and binary tree maps otherwise
* binary tree maps insertion/fetch speed enhanced by use of fast hashing on keys
* no data copying, source text data is never copied
* type conversions done on a per-request basis

**jsonic** does not make use of any particular instruction set/platform-specific optimization, so should have portable performance on all rust-compatible systems.

### Example use

```rust
fn main() {
    let json = "{\"jsonic\": \"Fast, small JSON parsing library for rust with no dependencies\"}";

    match jsonic::parse(json) {
        Ok(parsed) => { println!("Describe jsonic? {:?}", parsed["jsonic"].as_str()); }
        Err(error) => { eprintln!("{}", error); }
    }
}
```

### Benchmark

To get an overview of **jsonic**'s parsing performance compared to other JSON parsing engines written in rust, use ```cargo bench```.

Here is a sample run on an Apple iMac M1, 8GB RAM, macOS Sonoma:

```shell
     Running benches/json-rust.rs (target/release/deps/json_rust-0d2370885fb224f4)
Timer precision: 41 ns
json_rust                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ parse                                              │               │               │               │         │
   ├─ ./benches/data/canada.json        5.66 ms       │ 6.038 ms      │ 5.781 ms      │ 5.764 ms      │ 100     │ 100
   ├─ ./benches/data/citm_catalog.json  2.028 ms      │ 2.309 ms      │ 2.05 ms       │ 2.055 ms      │ 100     │ 100
   ╰─ ./benches/data/twitter.json       823.4 µs      │ 1.027 ms      │ 832 µs        │ 835.4 µs      │ 100     │ 100

     Running benches/jsonic.rs (target/release/deps/jsonic-0031ab613974bf81)
Timer precision: 41 ns
jsonic                                  fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ parse                                              │               │               │               │         │
   ├─ ./benches/data/canada.json        2.519 ms      │ 3.089 ms      │ 2.549 ms      │ 2.565 ms      │ 100     │ 100
   ├─ ./benches/data/citm_catalog.json  1.453 ms      │ 1.633 ms      │ 1.477 ms      │ 1.479 ms      │ 100     │ 100
   ╰─ ./benches/data/twitter.json       555.3 µs      │ 658.9 µs      │ 560.5 µs      │ 563.2 µs      │ 100     │ 100

     Running benches/serde_json.rs (target/release/deps/serde_json-b37e83074b30325e)
Timer precision: 41 ns
serde_json                              fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ parse                                              │               │               │               │         │
   ├─ ./benches/data/canada.json        4.489 ms      │ 4.977 ms      │ 4.561 ms      │ 4.58 ms       │ 100     │ 100
   ├─ ./benches/data/citm_catalog.json  2.177 ms      │ 2.961 ms      │ 2.223 ms      │ 2.231 ms      │ 100     │ 100
   ╰─ ./benches/data/twitter.json       1.031 ms      │ 1.203 ms      │ 1.041 ms      │ 1.046 ms      │ 100     │ 100

     Running benches/simd-json.rs (target/release/deps/simd_json-274159a472ebf0bc)
Timer precision: 41 ns
simd_json                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ parse                                              │               │               │               │         │
   ├─ ./benches/data/canada.json        5.494 ms      │ 6.828 ms      │ 5.709 ms      │ 5.716 ms      │ 100     │ 100
   ├─ ./benches/data/citm_catalog.json  2.695 ms      │ 3.883 ms      │ 2.766 ms      │ 2.78 ms       │ 100     │ 100
   ╰─ ./benches/data/twitter.json       1.114 ms      │ 1.726 ms      │ 1.127 ms      │ 1.162 ms      │ 100     │ 100

     Running benches/sonic-rs.rs (target/release/deps/sonic_rs-0ab724f2d9eb477b)
Timer precision: 41 ns
sonic_rs                                fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ parse                                              │               │               │               │         │
   ├─ ./benches/data/canada.json        4.104 ms      │ 4.794 ms      │ 4.196 ms      │ 4.217 ms      │ 100     │ 100
   ├─ ./benches/data/citm_catalog.json  1.783 ms      │ 2.662 ms      │ 1.815 ms      │ 1.845 ms      │ 100     │ 100
   ╰─ ./benches/data/twitter.json       817.2 µs      │ 1.025 ms      │ 834 µs        │ 834.5 µs      │ 100     │ 100
```


