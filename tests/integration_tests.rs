use rust_addr_clean::{clean_address, STREET_SUFFIX_ABBREVS};
use std::fs::read_to_string;

#[test]
fn test_clean_address() {
    let data = read_to_string("tests/test_addrs.txt").expect("Could not read file");

    for (i, line) in data.lines().enumerate() {
        let cleaned = clean_address(line, &STREET_SUFFIX_ABBREVS);
        // Add assertions
        // all uppercase
        // no nonalphanum and spaces
        // no instances or replaced values
        println!("Test {}: Cleaned address is {}", i + 1, cleaned);
    }
}
