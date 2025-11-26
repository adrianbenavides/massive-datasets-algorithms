use proptest::prelude::*;
use sketches::filters::bloom::BloomFilter;
use sketches::filters::traits::ApproximateMembershipQuery;
use sketches::hashing::AHasher;

proptest! {
    /// Property: Standard Bloom filter has no false negatives
    /// For any set of items inserted, all must be found on lookup
    #[test]
    fn bloom_no_false_negatives(
        items in prop::collection::vec(any::<u64>(), 0..1000)
    ) {
        let mut filter = BloomFilter::new(
            items.len() + 100,
            0.01,
            AHasher::default()
        );

        for item in &items {
            filter.insert(item);
        }

        for item in &items {
            prop_assert!(filter.contains(item), "False negative for item: {}", item);
        }
    }


    /// Property: Empty filter should not contain any items
    #[test]
    fn bloom_empty_contains_nothing(
        items in prop::collection::vec(any::<u64>(), 1..100)
    ) {
        let filter = BloomFilter::<u64, AHasher>::new(100, 0.01, AHasher::default());

        // An empty filter might still return true due to hash collisions
        // but statistically should have very low false positive rate
        let mut false_positives = 0;
        for item in &items {
            if filter.contains(item) {
                false_positives += 1;
            }
        }

        // With FPR of 0.01, we expect ~1 false positive per 100 queries
        // Allow up to 10% of items to be false positives (very generous)
        let fpr = false_positives as f64 / items.len() as f64;
        prop_assert!(fpr < 0.1, "Too many false positives in empty filter: {}", fpr);
    }

    /// Property: Filter length increases monotonically
    #[test]
    fn bloom_length_increases(
        items in prop::collection::vec(any::<u64>(), 1..100)
    ) {
        let mut filter = BloomFilter::new(items.len() + 10, 0.01, AHasher::default());
        let mut prev_len = 0;

        for item in &items {
            filter.insert(item);
            let new_len = filter.len();
            prop_assert!(new_len > prev_len, "Length did not increase after insert");
            prev_len = new_len;
        }

        prop_assert_eq!(filter.len(), items.len());
    }

    /// Property: Inserting duplicate items still works (idempotent for queries)
    #[test]
    fn bloom_duplicate_inserts(
        items in prop::collection::vec(any::<u64>(), 1..50)
    ) {
        let mut filter = BloomFilter::new(items.len() * 3, 0.01, AHasher::default());

        // Insert each item multiple times
        for item in &items {
            filter.insert(item);
            filter.insert(item); // duplicate
            filter.insert(item); // triplicate
        }

        // All items should still be found
        for item in &items {
            prop_assert!(filter.contains(item));
        }
    }

    /// Property: Different items with same hash behavior
    #[test]
    fn bloom_handles_hash_collisions(
        items in prop::collection::vec(any::<u32>(), 10..100)
    ) {
        let mut filter = BloomFilter::new(items.len() + 50, 0.01, AHasher::default());

        for item in &items {
            filter.insert(item);
        }

        for item in &items {
            prop_assert!(filter.contains(item));
        }
    }

    /// Property: Capacity is respected
    #[test]
    fn bloom_capacity_invariant(
        capacity in 10usize..1000,
        fpr in 0.001f64..0.1
    ) {
        let filter = BloomFilter::<u64, AHasher>::new(capacity, fpr, AHasher::default());
        prop_assert_eq!(filter.capacity(), capacity);
        prop_assert!((filter.false_positive_rate() - fpr).abs() < 1e-10);
    }
}
