use rust_addr_clean::{clean_address, REPLACEMENTS};
use std::fs::read_to_string;

#[test]
fn test_clean_address() {
    // Read the test addresses from the file
    let data = read_to_string("tests/test_addrs.txt").expect("Could not read file");

    // Assuming one address per line
    for (i, line) in data.lines().enumerate() {
        let cleaned = clean_address(line, &REPLACEMENTS);
        // Your assertions here...
        println!("Test {}: Cleaned address is {}", i + 1, cleaned);
    }
}
