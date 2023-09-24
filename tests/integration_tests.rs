use rust_addr_clean::{clean_address, clean_addresses, SINGLE_WORD_ABBREVS};
use std::fs::read_to_string;

#[test]
fn test_clean_address() {
    let data = read_to_string("tests/test_addrs.txt").expect("Could not read file");

    for (i, line) in data.lines().enumerate() {
        let cleaned = clean_address(line, &SINGLE_WORD_ABBREVS);
        // Add assertions
        // all uppercase
        // no nonalphanum and spaces
        // no instances or replaced values
        println!("Test {}: Cleaned address is {}", i + 1, cleaned);
    }
}

#[test]
fn test_clean_addresses() {
    let data = read_to_string("tests/test_addrs.txt").expect("Could not read file");
    let data: Vec<&str> = data.lines().collect();
    let cleaned = clean_addresses(data, &SINGLE_WORD_ABBREVS);
    // Add assertions
    // all uppercase
    // no nonalphanum and spaces
    // no instances or replaced values
    for (i, line) in cleaned.iter().enumerate() {
        println!("Test {}: Cleaned address is {}", i + 1, line);
    }
}
