#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref REPLACEMENTS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("AVE", "AVENUE");
        m.insert("ST", "STREET");
        m
    };
}

pub fn clean_address(address: &str, replacements: &HashMap<&str, &str>) -> String {
    // periods, hyphens, and slashes are significant and should not be removed
    // except for hyphens in place names, which should be replaced with a space
    let included_nonalphanum: [char; 3] = ['-', '.', '/'];

    let cleaned: String = address
        .chars()
        .filter(|&c| c.is_alphanumeric() || c.is_whitespace() || included_nonalphanum.contains(&c))
        .collect();

    let uppercased = cleaned.to_uppercase();

    uppercased
        .split_whitespace()
        .map(|word| *replacements.get(word).unwrap_or(&word))
        .collect::<Vec<&str>>()
        .join(" ")
}
