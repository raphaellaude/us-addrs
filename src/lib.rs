use crfsuite::Attribute;
use lazy_static::lazy_static;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;
mod abbreviations;
pub mod train;

use abbreviations::{DIRECTIONALS, STREET_NAMES};

pub enum Tag {
    AddressNumberPrefix,
    AddressNumber,
    AddressNumberSuffix,
    StreetNamePreModifier,
    StreetNamePreDirectional,
    StreetNamePreType,
    StreetName,
    StreetNamePostType,
    StreetNamePostDirectional,
    SubaddressType,
    SubaddressIdentifier,
    BuildingName,
    OccupancyType,
    OccupancyIdentifier,
    CornerOf,
    LandmarkName,
    PlaceName,
    StateName,
    ZipCode,
    USPSBoxType,
    USPSBoxID,
    USPSBoxGroupType,
    USPSBoxGroupID,
    IntersectionSeparator,
    Recipient,
    NotAddress,
}

lazy_static! {
    pub static ref MODEL: crfsuite::Model =
        crfsuite::Model::from_file("model/test_usaddr.crfsuite").unwrap();
}

/// Parse an unstructured U.S. address string into address components.
pub fn parse(address: &str) -> Vec<(String, String)> {
    let tokens = tokenize(address);
    let xseq = get_address_features(&tokens);

    let mut tagger = MODEL.tagger().unwrap();
    let tags = tagger.tag(&xseq).unwrap();

    zip_tokens_and_tags(tokens, tags)
}

pub fn zip_tokens_and_tags(tokens: Vec<String>, tags: Vec<String>) -> Vec<(String, String)> {
    tokens.into_iter().zip(tags.into_iter()).collect()
}

pub fn get_address_features(tokens: &Vec<String>) -> Vec<Vec<Attribute>> {
    let xseq = tokens
        .iter()
        .map(|token| get_token_features(token))
        .collect();

    add_feature_context(xseq)
}

pub fn add_feature_context(features: Vec<Vec<Attribute>>) -> Vec<Vec<Attribute>> {
    let mut features = features;

    if !features.is_empty() {
        features[0].push(Attribute::new("address.start", 1f64));
        features
            .last_mut()
            .unwrap()
            .push(Attribute::new("address.end", 1f64));
    }

    let n_features = features.len();

    // 1. Collect new attributes
    let mut new_attributes = Vec::new();
    for idx in 0..n_features {
        let mut current_attrs = Vec::new();
        if idx == 0 {
            current_attrs.extend(get_new_attributes(&features[idx + 1], "next"));
        } else if idx == 1 {
            current_attrs.push(Attribute::new("previous.address.start", 1f64));
        } else if idx == n_features - 2 {
            current_attrs.push(Attribute::new("next.address.end", 1f64));
        } else if idx == n_features - 1 {
            current_attrs.extend(get_new_attributes(&features[idx - 1], "previous"));
        } else {
            current_attrs.extend(get_new_attributes(&features[idx + 1], "next"));
            current_attrs.extend(get_new_attributes(&features[idx - 1], "previous"));
        }
        new_attributes.push(current_attrs);
    }

    for (idx, attrs) in new_attributes.into_iter().enumerate() {
        features[idx].extend(attrs);
    }
    features
}

fn get_new_attributes(feature: &Vec<Attribute>, prefix: &str) -> Vec<Attribute> {
    feature
        .iter()
        .map(|feature| Attribute::new(&format!("{}_{}", prefix, feature.name), feature.value))
        .collect()
}

pub fn tokenize(address: &str) -> Vec<String> {
    let address: String = clean_address(address);

    address
        .split([' ', ',', ';', ')', '\n'].as_ref())
        .filter(|x| !x.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn get_token_features(token: &str) -> Vec<Attribute> {
    let mut n_chars = 0;
    let mut numeric_digits = 0;
    let mut has_vowels = false;
    let mut last_char = None;

    for c in token.chars() {
        n_chars += 1;
        if c.is_numeric() {
            numeric_digits += 1;
        }
        if "aeiou".contains(c) {
            has_vowels = true;
        }
        last_char = Some(c);
    }

    let token_clean = token
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    let endsinpunc = last_char.map_or(false, |c| c.is_ascii_punctuation());
    let ends_in_period = last_char.map_or(false, |c| c == '.');
    let trailing_zeros = last_char.map_or(false, |c| c == '0');
    let digits = match numeric_digits {
        d if d == n_chars => "all_digits",
        d if d > 0 => "some_digits",
        _ => "no_digits",
    };

    // Utility function to push features based on a condition
    let add_feature = |features: &mut Vec<Attribute>, key: &str, condition: bool| {
        features.push(Attribute::new(key, if condition { 1f64 } else { 0f64 }));
    };

    let mut features = vec![
        Attribute::new(
            &format!("digits={}", digits),
            match numeric_digits {
                d if d > 0 => 1f64,
                _ => 0f64,
            },
        ),
        Attribute::new(
            &format!("word={}", token_clean),
            if token.chars().any(|c| c.is_alphabetic()) {
                1f64
            } else {
                0f64
            },
        ),
        Attribute::new(
            &format!(
                "length={}:{}",
                if digits == "all_digits" { "d" } else { "w" },
                numeric_digits
            ),
            1f64,
        ),
        Attribute::new("endsinpunc", endsinpunc as u8 as f64),
        Attribute::new("abbrev", ends_in_period as u8 as f64),
        Attribute::new("trailing.zeros", trailing_zeros as u8 as f64),
    ];

    add_feature(
        &mut features,
        "street_name",
        make_replacements(&token.to_lowercase(), &STREET_NAMES),
    );
    add_feature(
        &mut features,
        "directional",
        make_replacements(&token.to_lowercase(), &DIRECTIONALS),
    );
    add_feature(&mut features, "has.vowels", has_vowels);

    features
}

fn make_replacements(token: &str, replacements: &HashMap<&str, &str>) -> bool {
    replacements.get(&token).is_some()
}

/// Cleans a U.S. Address, applying the following transformations:
/// - All characters are converted to uppercase
/// - Extra whitespace is removed
/// - All non-ascii characters are removed
/// - All punctuation is removed, except for periods, hyphens, and slashes EXCEPT when
///  they are surrounded by numbers, in which case they are retained
pub fn clean_address(address: &str) -> String {
    address.trim().chars().nfkd().collect() // filter to ascii characters only, by closest
}

pub fn clean_addresses(addresses: Vec<&str>) -> Vec<String> {
    addresses
        .iter() // .iter is 42% faster than .par_iter()
        .map(|address| clean_address(address))
        .collect()
}

pub fn remove_insignificant_punctuation(address: &str) -> String {
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
            let sig_alphanum = ['-', '.', '/'].contains(&c); // and '#', '&', '½' ?

            if prev.is_numeric() && next.is_numeric() && sig_alphanum {
                output.push(c);
                continue;
            }
        }
    }
    output
}
