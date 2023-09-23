#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref REPLACEMENTS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("AVE", "AVENUE");
        m.insert("ST", "STREET");
        m
    };
}

fn clean_address(address: &str, replacements: &HashMap<&str, &str>) -> String {
    address
        .split_whitespace()
        .map(|word| *replacements.get(word).unwrap_or(&word))
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    let address = "123 MAIN ST";
    let cleaned_address = clean_address(address, &REPLACEMENTS);
    println!("Cleaned address: {}", cleaned_address);
}
