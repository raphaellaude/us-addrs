use std::fs::read_to_string;
use us_addrs::{
    add_feature_context, clean_address, clean_addresses, get_token_features, parse, tokenize,
};

#[test]
fn test_parse() {
    let data = read_to_string("tests/test_addrs.txt").expect("Could not read file");
    let data: Vec<&str> = data.lines().collect();
    for (i, line) in data.iter().enumerate() {
        let parsed = parse(line);
        println!("Test {}: Parsed address is {:?}", i + 1, parsed);
    }
}

#[test]
fn test_add_feature_context() {
    let data = read_to_string("tests/test_addrs.txt").expect("Could not read file");
    let data: Vec<&str> = data.lines().collect();
    for addr in data {
        let tokens = tokenize(addr);
        let mut xseq = Vec::new();

        for token in &tokens {
            let features = get_token_features(&token);
            xseq.push(features);
        }

        let xseq = add_feature_context(xseq);
        println!("Test: Feature context is {:?}", xseq);
    }
}

#[test]
fn test_tokenize() {
    let data = read_to_string("tests/test_addrs.txt").expect("Could not read file");
    let data: Vec<&str> = data.lines().collect();
    for (i, line) in data.iter().enumerate() {
        let tokens = tokenize(line);
        println!("Test {}: Tokens are {:?}", i + 1, tokens);
    }
}

#[test]
fn test_clean_address() {
    let data = read_to_string("tests/test_addrs.txt").expect("Could not read file");

    for (i, line) in data.lines().enumerate() {
        let cleaned = clean_address(line);
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
    let cleaned = clean_addresses(data);
    // Add assertions
    // all uppercase
    // no nonalphanum and spaces
    // no instances or replaced values
    for (i, line) in cleaned.iter().enumerate() {
        println!("Test {}: Cleaned address is {}", i + 1, line);
    }
}
