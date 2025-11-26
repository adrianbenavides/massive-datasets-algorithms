use sketches::filters::bloom::BloomFilter;
use sketches::filters::traits::ApproximateMembershipQuery;
use sketches::hashing::AHasher;

#[test]
fn test_bloom_fpr_within_bounds() {
    let n = 10_000;
    let f = 0.01;
    let mut filter = BloomFilter::new(n, f, AHasher::default());

    // Insert n items
    for i in 0..n as u64 {
        filter.insert(&i);
    }

    // Query 100k items not in filter
    let m = 100_000;
    let mut false_positives = 0;
    for i in n as u64..(n as u64 + m) {
        if filter.contains(&i) {
            false_positives += 1;
        }
    }

    let empirical_fpr = false_positives as f64 / m as f64;
    println!(
        "Standard Bloom - Empirical FPR: {:.4}, Theoretical: {:.4}",
        empirical_fpr, f
    );

    // Allow 20% deviation (generous for small sample)
    assert!((empirical_fpr - f).abs() <= f * 0.2);
}
