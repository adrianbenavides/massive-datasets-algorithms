use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::Rng;
use sketches::filters::bloom::BloomFilter;
use sketches::filters::traits::ApproximateMembershipQuery;
use sketches::hashing::AHasher;
use std::hint::black_box;

fn bloom_insertion(c: &mut Criterion) {
    let mut group = c.benchmark_group("bloom_insertion");

    for size in [10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut filter = BloomFilter::<_, AHasher>::new(size, 0.01);
            let mut rng = rand::rng();
            b.iter(|| {
                let item: u64 = rng.random();
                filter.insert(black_box(&item));
            });
        });
    }
    group.finish();
}

fn bloom_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("bloom_query");

    for size in [10_000, 100_000, 1_000_000] {
        let mut filter = BloomFilter::<_, AHasher>::new(size, 0.01);
        for i in 0..size as u64 {
            filter.insert(&i);
        }

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            let mut rng = rand::rng();
            b.iter(|| {
                let item: u64 = rng.random();
                black_box(filter.contains(&item));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bloom_insertion, bloom_query);
criterion_main!(benches);
