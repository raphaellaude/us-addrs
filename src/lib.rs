#[macro_use]
extern crate lazy_static;
extern crate unicode_normalization;
use rayon::prelude::*;
use std::collections::HashMap;

// module imports
mod abbreviations;
use unicode_normalization::UnicodeNormalization;

pub use crate::abbreviations::constants::{PHRASE_ABBREVS, SINGLE_WORD_ABBREVS};

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

/// Cleans a U.S. Address, applying the following transformations:
/// - All characters are converted to uppercase
/// - Extra whitespace is removed
/// - All non-ascii characters are removed
/// - All punctuation is removed, except for periods, hyphens, and slashes EXCEPT when
///  they are surrounded by numbers, in which case they are retained
/// - Single word street and geographic abbreviations are applied, e.g. "STREET" -> "ST",
/// "AVENUE" -> "AVE", "NORTH" -> "N", "SOUTHWEST" -> "SW", etc.
pub fn clean_address(address: &str, replacements: &HashMap<&str, &str>) -> String {
    let address: String = address.to_uppercase();

    let address: String = address
        .trim()
        .chars()
        .nfkd() // filter to ascii characters only, by closest
        .collect();

    let address: String = remove_insignificant_punctuation(&address);

    address
        .par_split_whitespace()
        .map(|word| *replacements.get(word).unwrap_or(&word))
        .collect::<Vec<&str>>()
        .join(" ")
}

pub fn clean_addresses(addresses: Vec<&str>, replacements: &HashMap<&str, &str>) -> Vec<String> {
    addresses
        .par_iter()
        .map(|address| clean_address(address, replacements))
        .collect()
}
