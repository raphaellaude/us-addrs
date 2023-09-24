#[macro_use]
extern crate lazy_static;
extern crate unicode_normalization;
use std::collections::HashMap;

// module imports
mod abbreviations;
use unicode_normalization::UnicodeNormalization;

pub use crate::abbreviations::constants::STREET_SUFFIX_ABBREVS;

pub fn remove_insignificant_punctuation(address: &str) -> String {
    let included_nonalphanum: [char; 3] = ['-', '.', '/'];

    let mut output = String::new();
    let chars: Vec<char> = address.chars().collect();

    for i in 0..chars.len() {
        let c = chars[i];

        if c.is_alphanumeric() || c.is_whitespace() {
            output.push(c);
            continue;
        }

        if i > 0 && i < chars.len() - 1 {
            let prev = chars[i - 1];
            let next = chars[i + 1];

            // periods, hyphens, and slashes are significant and should not be removed
            // except for hyphens in place names, which should be replaced with a space
            let sig_alphanum = included_nonalphanum.contains(&c);

            if prev.is_numeric() && next.is_numeric() && sig_alphanum {
                output.push(c);
                continue;
            }

            if sig_alphanum {
                output.push(' ');
                continue;
            }
        }
    }
    output
}

pub fn clean_address(address: &str, replacements: &HashMap<&str, &str>) -> String {
    let address: String = address
        .trim()
        .chars()
        .nfkd() // filter to ascii characters only, by closest
        .collect();

    let address: String = remove_insignificant_punctuation(&address);
    let address: String = address.to_uppercase();

    address
        .split_whitespace()
        .map(|word| *replacements.get(word).unwrap_or(&word))
        .collect::<Vec<&str>>()
        .join(" ")
}
