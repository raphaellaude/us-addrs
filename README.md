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
let addresses = read_to_string("tests/test_addrs.txt").expect("Could not read file");
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
