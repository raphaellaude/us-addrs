# us-addrs

US Addrs is a rust crate for parsing unstructured United States address strings into address components.
It is a rust implementation of the awesome [usaddress](https://github.com/datamade/usaddress/tree/master) library.
Thank you to the folks at [datamade](https://datamade.us/) for releasing such a cool tool.

US Addrs is currently _79% (~5x) faster_ than usaddress, though additional optimizations should be possible. Accuracy stats TK.
The goal of this implementation is to faciliate use cases requiring better performance, such as geocoding large batches of addresses.

:warning: This crate is under **active development** and may not match usaddress's accuracy. US Addrs will be better tested / documented shortly.

## Examples

US Addrs can be run from the command line

```bash
cargo run -- parse --address '33 Nassau Avenue, Brooklyn, NY'
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/us_addrs parse --address '33 Nassau Avenue, Brooklyn, NY'`
[("33", "AddressNumber"), ("Nassau", "StreetName"), ("Avenue", "StreetNamePostType"), ("Brooklyn", "PlaceName"), ("NY", "StateName")]
```

or by importing the crate and using the `parse` function

```rust
let addresses = read_to_string("tests/test_data/test_addrs.txt").expect("Could not read file");
let addresses: Vec<&str> = addresses.lines().collect();
for (i, addr) in addresses.iter().enumerate() {
    let parsed = parse(addr);
    println!("Test {}: Parsed address is {:?}", i + 1, parsed);
}
```

```
Test 1: Parsed address is [("123", "AddressNumber"), ("Elm", "StreetName"), ("St.", "StreetNamePostType"), ("Apt.", "OccupancyType"), ("4", "OccupancyIdentifier"), ("New", "PlaceName"), ("York", "PlaceName"), ("NY", "StateName"), ("10001", "ZipCode")]
```

etc...

## Accuracy

Currently, the tool is very accurate for simple addresses, but mediocre at complex addresses. See test results:

```
running 4 tests
test test_simple_address_patterns ... ok
There were 34 mistagged address components of 782 (4.3%). 22 partially failed addresses of 103 (21.4%)
test test_labeled ... ok
There were 978 mistagged address components of 4627 (21.1%). 625 partially failed addresses of 687 (91.0%)
test test_us50_tagged ... ok
There were 151 mistagged address components of 20977 (0.7%). 139 partially failed addresses of 4120 (3.4%)
test test_synthetic_clean_osm_data ... ok
```
