/// Hasher Comparison Benchmarks
///
/// Compares the three available hash function implementations with BloomFilter:
/// - AHasher (ahash)
/// - XXHasher (xxhash3)
/// - Murmur3Hasher (murmur3)
///
/// Metrics: Insert throughput, query throughput (positive/negative lookups)
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use sketches::benchmarks::Dataset;
use sketches::filters::bloom::BloomFilter;
use sketches::filters::traits::ApproximateMembershipQuery;
use sketches::hashing::{AHasher, Murmur3Hasher, XXHasher};
use std::hint::black_box;


// ============================================================================
// Insert Benchmarks - Compare hasher performance during insertions
// ============================================================================

fn hasher_insert_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("hasher_insert_comparison");

    // Test with different dataset sizes
    for size in [1000, 10_000, 100_000] {
        let dataset = Dataset::uniform(size, 42);
        let fpr = 0.01;

        // AHasher
        group.bench_with_input(BenchmarkId::new("ahash", size), &dataset, |b, dataset| {
            b.iter(|| {
                let mut filter = BloomFilter::<_, AHasher>::new(dataset.inserted.len(), fpr);
                for item in &dataset.inserted {
                    filter.insert(black_box(item));
                }
                black_box(filter)
            });
        });

        // XXHasher
        group.bench_with_input(BenchmarkId::new("xxhash3", size), &dataset, |b, dataset| {
            b.iter(|| {
                let mut filter = BloomFilter::<_, XXHasher>::new(dataset.inserted.len(), fpr);
                for item in &dataset.inserted {
                    filter.insert(black_box(item));
                }
                black_box(filter)
            });
        });

        // Murmur3Hasher
        group.bench_with_input(BenchmarkId::new("murmur3", size), &dataset, |b, dataset| {
            b.iter(|| {
                let mut filter = BloomFilter::<_, Murmur3Hasher>::new(dataset.inserted.len(), fpr);
                for item in &dataset.inserted {
                    filter.insert(black_box(item));
                }
                black_box(filter)
            });
        });
    }

    group.finish();
}

// ============================================================================
// Query Benchmarks - Compare hasher performance during lookups
// ============================================================================

fn hasher_query_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("hasher_query_comparison");

    // Test with different dataset sizes
    for size in [1000, 10_000, 100_000] {
        let dataset = Dataset::uniform(size, 42);
        let fpr = 0.01;

        // Pre-build filters
        let mut filter_ahash = BloomFilter::<_, AHasher>::new(dataset.inserted.len(), fpr);
        let mut filter_xxhash = BloomFilter::<_, XXHasher>::new(dataset.inserted.len(), fpr);
        let mut filter_murmur3 = BloomFilter::<_, Murmur3Hasher>::new(dataset.inserted.len(), fpr);

        for item in &dataset.inserted {
            filter_ahash.insert(item);
            filter_xxhash.insert(item);
            filter_murmur3.insert(item);
        }

        // Combine positive and negative queries
        let query_items: Vec<u64> = dataset
            .queries_present
            .iter()
            .chain(dataset.queries_absent.iter())
            .copied()
            .collect();

        // AHasher queries
        group.bench_with_input(BenchmarkId::new("ahash", size), &query_items, |b, items| {
            let mut idx = 0;
            b.iter(|| {
                let item = &items[idx % items.len()];
                idx += 1;
                black_box(filter_ahash.contains(black_box(item)))
            });
        });

        // XXHasher queries
        group.bench_with_input(
            BenchmarkId::new("xxhash3", size),
            &query_items,
            |b, items| {
                let mut idx = 0;
                b.iter(|| {
                    let item = &items[idx % items.len()];
                    idx += 1;
                    black_box(filter_xxhash.contains(black_box(item)))
                });
            },
        );

        // Murmur3Hasher queries
        group.bench_with_input(
            BenchmarkId::new("murmur3", size),
            &query_items,
            |b, items| {
                let mut idx = 0;
                b.iter(|| {
                    let item = &items[idx % items.len()];
                    idx += 1;
                    black_box(filter_murmur3.contains(black_box(item)))
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Positive Query Benchmarks - Test with known items (true positives)
// ============================================================================

fn hasher_positive_query_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("hasher_positive_queries");

    for size in [1000, 10_000, 100_000] {
        let dataset = Dataset::uniform(size, 42);
        let fpr = 0.01;

        // Pre-build filters
        let mut filter_ahash = BloomFilter::<_, AHasher>::new(dataset.inserted.len(), fpr);
        let mut filter_xxhash = BloomFilter::<_, XXHasher>::new(dataset.inserted.len(), fpr);
        let mut filter_murmur3 = BloomFilter::<_, Murmur3Hasher>::new(dataset.inserted.len(), fpr);

        for item in &dataset.inserted {
            filter_ahash.insert(item);
            filter_xxhash.insert(item);
            filter_murmur3.insert(item);
        }

        // Query only items we know are present
        let query_items = &dataset.queries_present;

        // AHasher
        group.bench_with_input(BenchmarkId::new("ahash", size), query_items, |b, items| {
            let mut idx = 0;
            b.iter(|| {
                let item = &items[idx % items.len()];
                idx += 1;
                black_box(filter_ahash.contains(black_box(item)))
            });
        });

        // XXHasher
        group.bench_with_input(
            BenchmarkId::new("xxhash3", size),
            query_items,
            |b, items| {
                let mut idx = 0;
                b.iter(|| {
                    let item = &items[idx % items.len()];
                    idx += 1;
                    black_box(filter_xxhash.contains(black_box(item)))
                });
            },
        );

        // Murmur3Hasher
        group.bench_with_input(
            BenchmarkId::new("murmur3", size),
            query_items,
            |b, items| {
                let mut idx = 0;
                b.iter(|| {
                    let item = &items[idx % items.len()];
                    idx += 1;
                    black_box(filter_murmur3.contains(black_box(item)))
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Negative Query Benchmarks - Test with absent items (false positive testing)
// ============================================================================

fn hasher_negative_query_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("hasher_negative_queries");

    for size in [1000, 10_000, 100_000] {
        let dataset = Dataset::uniform(size, 42);
        let fpr = 0.01;

        // Pre-build filters
        let mut filter_ahash = BloomFilter::<_, AHasher>::new(dataset.inserted.len(), fpr);
        let mut filter_xxhash = BloomFilter::<_, XXHasher>::new(dataset.inserted.len(), fpr);
        let mut filter_murmur3 = BloomFilter::<_, Murmur3Hasher>::new(dataset.inserted.len(), fpr);

        for item in &dataset.inserted {
            filter_ahash.insert(item);
            filter_xxhash.insert(item);
            filter_murmur3.insert(item);
        }

        // Query only items we know are absent
        let query_items = &dataset.queries_absent;

        // AHasher
        group.bench_with_input(BenchmarkId::new("ahash", size), query_items, |b, items| {
            let mut idx = 0;
            b.iter(|| {
                let item = &items[idx % items.len()];
                idx += 1;
                black_box(filter_ahash.contains(black_box(item)))
            });
        });

        // XXHasher
        group.bench_with_input(
            BenchmarkId::new("xxhash3", size),
            query_items,
            |b, items| {
                let mut idx = 0;
                b.iter(|| {
                    let item = &items[idx % items.len()];
                    idx += 1;
                    black_box(filter_xxhash.contains(black_box(item)))
                });
            },
        );

        // Murmur3Hasher
        group.bench_with_input(
            BenchmarkId::new("murmur3", size),
            query_items,
            |b, items| {
                let mut idx = 0;
                b.iter(|| {
                    let item = &items[idx % items.len()];
                    idx += 1;
                    black_box(filter_murmur3.contains(black_box(item)))
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Zipfian Distribution Benchmarks - Real-world skewed data
// ============================================================================

fn hasher_zipfian_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("hasher_zipfian_distribution");

    // Use Zipfian distribution (common in real-world scenarios)
    let size = 100_000;
    let cardinality = 10_000;
    let alpha = 1.07; // Realistic for web traffic
    let dataset = Dataset::zipfian(size, cardinality, alpha, 42);
    let fpr = 0.01;

    // AHasher
    group.bench_function("ahash_insert", |b| {
        b.iter(|| {
            let mut filter = BloomFilter::<_, AHasher>::new(cardinality, fpr);
            for item in &dataset.inserted {
                filter.insert(black_box(item));
            }
            black_box(filter)
        });
    });

    // XXHasher
    group.bench_function("xxhash3_insert", |b| {
        b.iter(|| {
            let mut filter = BloomFilter::<_, XXHasher>::new(cardinality, fpr);
            for item in &dataset.inserted {
                filter.insert(black_box(item));
            }
            black_box(filter)
        });
    });

    // Murmur3Hasher
    group.bench_function("murmur3_insert", |b| {
        b.iter(|| {
            let mut filter = BloomFilter::<_, Murmur3Hasher>::new(cardinality, fpr);
            for item in &dataset.inserted {
                filter.insert(black_box(item));
            }
            black_box(filter)
        });
    });

    group.finish();
}

// ============================================================================
// Main
// ============================================================================

criterion_group!(
    benches,
    hasher_insert_comparison,
    hasher_query_comparison,
    hasher_positive_query_comparison,
    hasher_negative_query_comparison,
    hasher_zipfian_comparison,
);
criterion_main!(benches);
