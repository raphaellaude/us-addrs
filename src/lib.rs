use std::collections::HashMap;

use crfsuite::Attribute;
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

/// Parse an unstructured U.S. address string into address components.
pub fn parse(address: &str) -> Vec<(String, String)> {
    let tokens = tokenize(address);
    let xseq = get_address_features(&tokens);

    let model = crfsuite::Model::from_file("model/usaddr.crfsuite").unwrap();

    let mut tagger = model.tagger().unwrap();
    let _res = tagger.tag(&xseq).unwrap();

    tokens
        .into_iter()
        .zip(_res.iter())
        .map(|(token, tag)| (token, tag.to_string()))
        .collect()
}

pub fn get_address_features(tokens: &Vec<String>) -> Vec<Vec<Attribute>> {
    let mut xseq = Vec::new();

    for token in tokens {
        let features = get_token_features(token);
        xseq.push(features);
    }

    let xseq = add_feature_context(xseq);

    xseq
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
        .map(|feature| Attribute::new(&format!("{}.{}", prefix, feature.name), feature.value))
        .collect()
}

pub fn tokenize(address: &str) -> Vec<String> {
    let address: String = clean_address(address);
    let tokens: Vec<String> = address
        .split([' ', ',', ';', ')', '\n'])
        .filter(|&x| !x.is_empty())
        .map(|s| s.to_string())
        .collect();
    tokens
}

pub fn get_token_features(token: &str) -> Vec<Attribute> {
    let n_chars = token.chars().count();
    let numeric_digits = token.chars().filter(|c| c.is_numeric()).count();
    let has_vowels = token.chars().any(|c| "aeiou".contains(c));

    // Utility function to push features based on a condition
    let add_feature = |features: &mut Vec<Attribute>, key: &str, condition: bool| {
        features.push(Attribute::new(key, if condition { 1f64 } else { 0f64 }));
    };

    let mut features = vec![
        Attribute::new(
            "digits",
            match numeric_digits {
                d if d == n_chars => 1f64,
                d if d > 0 => 0.5f64,
                _ => 0f64,
            },
        ),
        Attribute::new(
            "word",
            if token.chars().any(|c| c.is_alphabetic()) {
                1f64
            } else {
                0f64
            },
        ),
        Attribute::new("length", n_chars as f64),
        Attribute::new(
            "endsinpinc",
            token
                .chars()
                .last()
                .map_or(0f64, |c| c.is_ascii_punctuation() as u8 as f64),
        ),
    ];

    add_feature(
        &mut features,
        "street_name",
        make_replacements(token, &STREET_NAMES),
    );
    add_feature(
        &mut features,
        "directional",
        make_replacements(token, &DIRECTIONALS),
    );
    add_feature(&mut features, "vowels", has_vowels);

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
/// - Single word street and geographic abbreviations are applied, e.g. "STREET" -> "ST",
/// "AVENUE" -> "AVE", "NORTH" -> "N", "SOUTHWEST" -> "SW", etc.
pub fn clean_address(address: &str) -> String {
    let address = address.to_uppercase();

    let address: String = address
        .trim()
        .chars()
        .nfkd() // filter to ascii characters only, by closest
        .collect();

    remove_insignificant_punctuation(&address)
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
            let sig_alphanum = ['-', '.', '/'].contains(&c);

            if prev.is_numeric() && next.is_numeric() && sig_alphanum {
                output.push(c);
                continue;
            }

            if sig_alphanum {
                output.push(' ');
                // continue;
            }
        }
    }
    output
}
