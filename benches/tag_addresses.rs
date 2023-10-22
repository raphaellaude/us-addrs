use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use us_addrs::parse;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("tag");

    // Read raw data
    let raw_data = fs::read_to_string("tests/us50.test.raw").unwrap();
    let data: Vec<&str> = raw_data.lines().collect();

    group.bench_function("crfs", |b| {
        b.iter(|| {
            for address in &data {
                let _parsed = parse(black_box(address));
            }
        });
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
