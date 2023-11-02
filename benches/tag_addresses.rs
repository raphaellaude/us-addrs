use crfsuite::Item;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use us_addrs::{get_address_features, parse, tokenize, zip_tokens_and_tags, MODEL};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("tag");

    // Read raw data
    let raw_data = fs::read_to_string("tests/test_data/us50.test.raw").unwrap();
    let data: Vec<&str> = raw_data.lines().collect();

    // Benchmark the whole function

    group.bench_function("crfsuite", |b| {
        b.iter(|| {
            for address in &data {
                let _parsed = parse(black_box(address));
            }
        });
    });

    // Profile it's components

    group.bench_function("tokenize", |b| {
        b.iter(|| {
            for address in &data {
                let _tokens = tokenize(black_box(address));
            }
        });
    });

    let data_tokens = data
        .iter()
        .map(|x| tokenize(x))
        .collect::<Vec<Vec<String>>>();

    group.bench_function("get_address_features", |b| {
        b.iter(|| {
            for tokens in &data_tokens {
                let _xseq = get_address_features(black_box(tokens));
            }
        });
    });

    let xseqs = data_tokens
        .iter()
        .map(|x| get_address_features(x))
        .collect::<Vec<Vec<Item>>>();

    group.bench_function("unwrap_model", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let _tagger = MODEL.tagger().unwrap();
            }
        });
    });

    let mut tagger = MODEL.tagger().unwrap();

    let address_tags = xseqs
        .iter()
        .map(|x| tagger.tag(x).unwrap())
        .collect::<Vec<Vec<String>>>();

    group.bench_function("zip_tokens_and_tags", |b| {
        b.iter(|| {
            for (tokens, tags) in data_tokens.iter().zip(address_tags.iter()) {
                let _zipped = zip_tokens_and_tags(tokens.to_vec(), tags.to_vec());
            }
        });
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
