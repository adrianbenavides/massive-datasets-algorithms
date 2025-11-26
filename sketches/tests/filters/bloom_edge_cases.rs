use rand::RngCore;
/// Edge case tests for Bloom filters
///
/// These tests cover boundary conditions, extreme inputs, and unusual scenarios
use sketches::filters::bloom::BloomFilter;
use sketches::filters::traits::ApproximateMembershipQuery;
use sketches::hashing::AHasher;

#[test]
#[should_panic(expected = "Capacity must be greater than 0")]
fn test_zero_capacity_bloom() {
    // Creating a Bloom filter with 0 capacity should panic due to invalid capacity
    let _ = BloomFilter::<u64, AHasher>::new(0, 0.01);
}

#[test]
fn test_very_low_fpr() {
    // Test with extremely low false positive rate
    let mut filter = BloomFilter::<_, AHasher>::new(100, 0.0001);

    for i in 0..100u64 {
        filter.insert(&i);
    }

    // All inserted items should be found
    for i in 0..100u64 {
        assert!(filter.contains(&i), "False negative for {}", i);
    }

    // Check FPR is configured correctly
    assert!((filter.false_positive_rate() - 0.0001).abs() < 1e-10);
}

#[test]
fn test_very_high_fpr() {
    // Test with high false positive rate (less optimal but valid)
    let mut filter = BloomFilter::<_, AHasher>::new(100, 0.5);

    for i in 0..50u64 {
        filter.insert(&i);
    }

    // Still no false negatives
    for i in 0..50u64 {
        assert!(filter.contains(&i));
    }
}

#[test]
fn test_large_capacity() {
    // Test with very large capacity
    let filter = BloomFilter::<u64, AHasher>::new(10_000_000, 0.01);
    assert_eq!(filter.capacity(), 10_000_000);
    assert_eq!(filter.len(), 0);
}

#[test]
fn test_single_item() {
    // Test with just one item
    let mut filter = BloomFilter::<_, AHasher>::new(1, 0.01);
    filter.insert(&42u64);

    assert!(filter.contains(&42u64));
    assert_eq!(filter.len(), 1);
}

#[test]
fn test_sequential_vs_random_insertions() {
    let mut filter1 = BloomFilter::<_, AHasher>::new(1000, 0.01);
    let mut filter2 = BloomFilter::<_, AHasher>::new(1000, 0.01);

    // Sequential insertions
    for i in 0..1000u64 {
        filter1.insert(&i);
    }

    // Random insertions
    let mut rng = rand::rng();
    let mut x: u64;
    let mut items = Vec::new();
    for _ in 0..1000 {
        x = rng.next_u64();
        items.push(x);
        filter2.insert(&x);
    }

    // Both should have no false negatives
    for i in 0..1000u64 {
        assert!(filter1.contains(&i));
    }

    for item in items {
        assert!(filter2.contains(&item));
    }
}

#[test]
fn test_extreme_hash_values() {
    // Test with values that might produce extreme hash values
    let mut filter = BloomFilter::<_, AHasher>::new(100, 0.01);

    let extreme_values = vec![
        0u64,
        1,
        u64::MAX,
        u64::MAX - 1,
        u64::MAX / 2,
        0x0F0F0F0F0F0F0F0F,
        0xAAAAAAAAAAAAAAAA,
        0x5555555555555555,
    ];

    for &val in &extreme_values {
        filter.insert(&val);
    }

    for &val in &extreme_values {
        assert!(filter.contains(&val), "False negative for {}", val);
    }
}

#[test]
fn test_string_items() {
    // Test that Bloom filter works with different types (not just u64)
    let mut filter = BloomFilter::<_, AHasher>::new(100, 0.01);

    let items = vec!["hello", "world", "bloom", "filter", "test"];

    for item in &items {
        filter.insert(item);
    }

    for item in &items {
        assert!(filter.contains(item), "False negative for {}", item);
    }
}

#[test]
fn test_capacity_vs_actual_insertions() {
    // Test what happens when we exceed capacity
    let capacity = 100;
    let mut filter = BloomFilter::<_, AHasher>::new(capacity, 0.01);

    // Insert more than capacity
    for i in 0..200u64 {
        filter.insert(&i);
    }

    assert_eq!(filter.len(), 200);

    // All items should still be found (no false negatives)
    for i in 0..200u64 {
        assert!(filter.contains(&i));
    }

    // But FPR might be higher than configured for non-inserted items
    // This is expected behavior
}
