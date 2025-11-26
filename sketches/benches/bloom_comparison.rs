/// Cross-crate filter benchmarks
///
/// Compares our Bloom filter implementations against popular Rust crates:
/// - fastbloom
/// - probabilistic-collections
/// - pdatastructs
///
/// Metrics: Insert throughput, query throughput, memory usage, FPR validation
use criterion::{Criterion, criterion_group, criterion_main};
use pdatastructs::filters::Filter as PdataFilter;
use sketches::benchmarks::Dataset;
use sketches::filters::bloom::BloomFilter;
use sketches::filters::traits::ApproximateMembershipQuery;
use sketches::hashing::AHasher;
use std::hint::black_box;

// ============================================================================
// Insert Benchmarks
// ============================================================================

fn filter_insert_comparison(c: &mut Criterion) {
    let dataset = Dataset::medium(42); // 100K items
    let n = dataset.inserted.len();
    let fpr = 0.01;

    let mut group = c.benchmark_group("filter_insert_100k");

    // Our standard Bloom filter
    group.bench_function("sketches_bloom", |b| {
        b.iter(|| {
            let mut filter = BloomFilter::<_, AHasher>::new(n, fpr);
            for item in &dataset.inserted {
                filter.insert(black_box(item));
            }
            black_box(filter)
        });
    });

    // fastbloom
    group.bench_function("fastbloom", |b| {
        b.iter(|| {
            let mut filter = fastbloom::BloomFilter::with_false_pos(fpr).expected_items(n);
            for item in &dataset.inserted {
                filter.insert(black_box(item));
            }
            black_box(filter)
        });
    });

    // probabilistic-collections
    group.bench_function("probabilistic_collections", |b| {
        b.iter(|| {
            let mut filter: probabilistic_collections::bloom::BloomFilter<u64> =
                probabilistic_collections::bloom::BloomFilter::new(n, fpr);
            for item in &dataset.inserted {
                filter.insert(black_box(item));
            }
            black_box(filter)
        });
    });

    // pdatastructs
    group.bench_function("pdatastructs", |b| {
        b.iter(|| {
            let mut filter =
                pdatastructs::filters::bloomfilter::BloomFilter::with_properties(n, fpr);
            for item in &dataset.inserted {
                let _ = filter.insert(black_box(&item.to_string()));
            }
            black_box(filter)
        });
    });

    group.finish();
}

// ============================================================================
// Query Benchmarks
// ============================================================================

fn filter_query_comparison(c: &mut Criterion) {
    let dataset = Dataset::medium(42);
    let n = dataset.inserted.len();
    let fpr = 0.01;

    let mut group = c.benchmark_group("filter_query");

    // Pre-build filters
    let mut sketches_bloom = BloomFilter::<_, AHasher>::new(n, fpr);
    let mut fastbloom_filter = fastbloom::BloomFilter::with_false_pos(fpr).expected_items(n);
    let mut prob_coll_filter: probabilistic_collections::bloom::BloomFilter<u64> =
        probabilistic_collections::bloom::BloomFilter::new(n, fpr);
    let mut pdatastructs_filter =
        pdatastructs::filters::bloomfilter::BloomFilter::with_properties(n, fpr);

    for item in &dataset.inserted {
        sketches_bloom.insert(item);
        fastbloom_filter.insert(item);
        prob_coll_filter.insert(item);
        let _ = pdatastructs_filter.insert(&item.to_string());
    }

    let query_items: Vec<u64> = dataset
        .queries_present
        .iter()
        .chain(dataset.queries_absent.iter())
        .copied()
        .collect();

    // Benchmark queries
    group.bench_function("sketches_bloom", |b| {
        let mut idx = 0;
        b.iter(|| {
            let item = &query_items[idx % query_items.len()];
            idx += 1;
            black_box(sketches_bloom.contains(black_box(item)))
        });
    });

    group.bench_function("fastbloom", |b| {
        let mut idx = 0;
        b.iter(|| {
            let item = &query_items[idx % query_items.len()];
            idx += 1;
            black_box(fastbloom_filter.contains(black_box(item)))
        });
    });

    group.bench_function("probabilistic_collections", |b| {
        let mut idx = 0;
        b.iter(|| {
            let item = &query_items[idx % query_items.len()];
            idx += 1;
            black_box(prob_coll_filter.contains(black_box(item)))
        });
    });

    group.bench_function("pdatastructs", |b| {
        let mut idx = 0;
        b.iter(|| {
            let item_str = query_items[idx % query_items.len()].to_string();
            idx += 1;
            black_box(pdatastructs_filter.query(black_box(&item_str)))
        });
    });

    group.finish();
}

// ============================================================================
// FPR Validation Benchmarks
// ============================================================================

fn filter_fpr_validation(c: &mut Criterion) {
    let dataset = Dataset::large(42); // 1M items for better FPR measurement
    let n = dataset.inserted.len();
    let fpr = 0.01;

    let mut group = c.benchmark_group("filter_fpr_validation");

    group.bench_function("sketches_bloom_build_and_measure", |b| {
        b.iter(|| {
            let mut filter = BloomFilter::<_, AHasher>::new(n, fpr);
            for item in &dataset.inserted {
                filter.insert(item);
            }

            // Measure false positives
            let mut false_positives = 0;
            for item in &dataset.queries_absent {
                if filter.contains(item) {
                    false_positives += 1;
                }
            }

            let empirical_fpr = false_positives as f64 / dataset.queries_absent.len() as f64;
            black_box((filter, empirical_fpr))
        });
    });

    group.finish();
}

// ============================================================================
// Main
// ============================================================================

criterion_group!(
    benches,
    filter_insert_comparison,
    filter_query_comparison,
    filter_fpr_validation,
);
criterion_main!(benches);
