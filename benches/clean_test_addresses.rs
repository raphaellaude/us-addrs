use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_addr_clean::{clean_address, clean_addresses, SINGLE_WORD_ABBREVS};
use std::fs::read_to_string;

fn clean_test_address(address: &str) {
    clean_address(black_box(address), &SINGLE_WORD_ABBREVS);
}

fn clean_address_batch(addresses: Vec<&str>) {
    clean_addresses(addresses, &SINGLE_WORD_ABBREVS);
}

fn bench(c: &mut Criterion) {
    c.bench_function("clean_test_address", |b| {
        b.iter(|| clean_test_address("15.2 North Spruce Road., Apt #2B, Washington, DC 20500"))
    });

    c.bench_function("clean_address_batch", |b| {
        let data = read_to_string("tests/test_addrs.txt").expect("Could not read file");
        let data: Vec<&str> = data.lines().collect();
        b.iter_batched(
            || data.clone(),
            |data| clean_address_batch(data),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
