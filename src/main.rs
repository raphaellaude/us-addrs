use std::collections::HashMap;

fn clean_address(address: &str, replacements: &HashMap<&str, &str>) -> String {
    address
        .split_whitespace()
        .map(|word| *replacements.get(word).unwrap_or(&word))
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    // Prepare the replacements mapping
    let mut replacements = HashMap::new();
    replacements.insert("AVE", "AVENUE");
    replacements.insert("ST", "STREET");
    replacements.insert("RD", "ROAD");

    // Test address
    let address = "123 MAIN ST";

    let cleaned_address = clean_address(address, &replacements);
    println!("Cleaned address: {}", cleaned_address); // Output should be "123 MAIN STREET"
}
