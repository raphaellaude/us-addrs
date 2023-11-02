use core::panic;
use std::collections::HashSet;

use us_addrs::{parse, read_xml_tagged_addresses};

fn get_erroring_addresses(file_path: &str) -> (Vec<String>, u32, HashSet<usize>, usize) {
    let mut errors = Vec::new();
    let mut n_tags = 0;
    let mut failed_addresses = HashSet::new();

    let (addresses, tags) = read_xml_tagged_addresses(file_path);
    for (i, address) in addresses.iter().enumerate() {
        let parsed = parse(address);
        // assert_eq!(parsed.len(), tags[i].len());
        if parsed.len() != tags[i].len() {
            errors.push(format!(
                "{}. Mismatched length: {} != {}",
                i,
                parsed.len(),
                tags[i].len()
            ));
        }
        for (j, tag) in tags[i].iter().enumerate() {
            n_tags += 1;
            // assert_eq!(parsed[j].1, tag.to_string());
            if parsed[j].1 != tag.to_string() {
                // errors.push((i, address.to_string(), parsed.clone(), tags[i].to_vec()));
                // errors.push((i, address.to_string(), parsed[j].1.clone(), tag.to_string()));
                errors.push(format!(
                    "{}. Mismatched tag: {} != {}",
                    i,
                    parsed[j].1,
                    tag.to_string()
                ));
                failed_addresses.insert(i);
            }
        }
    }
    (errors, n_tags, failed_addresses, addresses.len())
}

#[test]
fn test_simple_address_patterns() {
    let (errors, n_tags, failed_addresses, n_addresses) =
        get_erroring_addresses("tests/test_data/simple_address_patterns.xml");

    if !errors.is_empty() {
        panic!(
            "There were {} mistagged address components of {} ({:.1}%). {} partially failed addresses of {} ({:.1}%)",
            errors.len(),
            n_tags,
            errors.len() as f64 / n_tags as f64 * 100.0,
            failed_addresses.len(),
            n_addresses,
            failed_addresses.len() as f64 / n_addresses as f64 * 100.0,
        );
    }
}

#[test]
fn test_labeled() {
    let (errors, n_tags, failed_addresses, n_addresses) =
        get_erroring_addresses("tests/test_data/labeled.xml");

    if !errors.is_empty() {
        panic!(
            "There were {} mistagged address components of {} ({:.1}%). {} partially failed addresses of {} ({:.1}%)",
            errors.len(),
            n_tags,
            errors.len() as f64 / n_tags as f64 * 100.0,
            failed_addresses.len(),
            n_addresses,
            failed_addresses.len() as f64 / n_addresses as f64 * 100.0,
        );
    }
}

#[test]
fn test_us50_tagged() {
    let (errors, n_tags, failed_addresses, n_addresses) =
        get_erroring_addresses("tests/test_data/us50_test_tagged.xml");

    if !errors.is_empty() {
        panic!(
            "There were {} mistagged address components of {} ({:.1}%). {} partially failed addresses of {} ({:.1}%)",
            errors.len(),
            n_tags,
            errors.len() as f64 / n_tags as f64 * 100.0,
            failed_addresses.len(),
            n_addresses,
            failed_addresses.len() as f64 / n_addresses as f64 * 100.0,
        );
    }
}

#[test]
fn test_synthetic_clean_osm_data() {
    let (errors, n_tags, failed_addresses, n_addresses) =
        get_erroring_addresses("tests/test_data/synthetic_clean_osm_data.xml");

    if !errors.is_empty() {
        panic!(
            "There were {} mistagged address components of {} ({:.1}%). {} partially failed addresses of {} ({:.1}%)",
            errors.len(),
            n_tags,
            errors.len() as f64 / n_tags as f64 * 100.0,
            failed_addresses.len(),
            n_addresses,
            failed_addresses.len() as f64 / n_addresses as f64 * 100.0,
        );
    }
}
