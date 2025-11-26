/// Common datasets for benchmarking probabilistic data structures
///
/// Provides uniform and skewed (Zipfian) distributions for consistent
/// cross-crate benchmarking.
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand_distr::{Distribution, Zipf};

/// A dataset for benchmarking with inserted items and query sets
#[derive(Clone)]
pub struct Dataset {
    /// Items to insert into the data structure
    pub inserted: Vec<u64>,
    /// Query items that ARE in the inserted set (for testing true positives)
    pub queries_present: Vec<u64>,
    /// Query items that ARE NOT in the inserted set (for testing false positives)
    pub queries_absent: Vec<u64>,
}

impl Dataset {
    /// Generate a dataset with uniformly random items
    ///
    /// # Arguments
    ///
    /// * `n` - Number of items to insert
    /// * `seed` - Random seed for reproducibility
    ///
    /// # Examples
    ///
    /// ```
    /// use sketches_bench::benchmarks::datasets::Dataset;
    ///
    /// let dataset = Dataset::uniform(10_000, 42);
    /// assert_eq!(dataset.inserted.len(), 10_000);
    /// assert_eq!(dataset.queries_present.len(), 1_000); // 10% of inserted
    /// assert_eq!(dataset.queries_absent.len(), 1_000);
    /// ```
    pub fn uniform(n: usize, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);

        // Generate n unique items
        let inserted: Vec<u64> = (0..n).map(|_| rng.random()).collect();

        // Sample 10% of inserted items for positive queries
        let queries_present: Vec<u64> = inserted.iter().step_by(10).copied().take(n / 10).collect();

        // Generate items NOT in inserted set for negative queries
        let inserted_set: std::collections::HashSet<u64> = inserted.iter().copied().collect();

        let mut queries_absent = Vec::new();
        while queries_absent.len() < n / 10 {
            let item: u64 = rng.random();
            if !inserted_set.contains(&item) {
                queries_absent.push(item);
            }
        }

        Dataset {
            inserted,
            queries_present,
            queries_absent,
        }
    }

    /// Generate a dataset with Zipfian (power-law) distribution
    ///
    /// Common in real-world scenarios where a few items are very frequent
    /// and most items are rare (e.g., web traffic, word frequencies).
    ///
    /// # Arguments
    ///
    /// * `n` - Number of items to insert
    /// * `cardinality` - Number of unique items in the universe
    /// * `alpha` - Zipfian exponent (1.0 = classic Zipf, higher = more skewed)
    /// * `seed` - Random seed for reproducibility
    ///
    /// # Examples
    ///
    /// ```
    /// use sketches_bench::benchmarks::datasets::Dataset;
    ///
    /// // Generate 100K insertions from a universe of 10K unique items
    /// // with Zipfian distribution (alpha=1.07 is realistic for web traffic)
    /// let dataset = Dataset::zipfian(100_000, 10_000, 1.07, 42);
    /// assert_eq!(dataset.inserted.len(), 100_000);
    /// // queries_present will sample from the inserted (with duplicates)
    /// // queries_absent will be from the universe but not frequently inserted
    /// ```
    pub fn zipfian(n: usize, cardinality: usize, alpha: f64, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let zipf = Zipf::new(cardinality as f64, alpha).expect("Invalid Zipfian parameters");

        // Generate n items following Zipfian distribution
        let inserted: Vec<u64> = (0..n).map(|_| zipf.sample(&mut rng) as u64).collect();

        // Sample 10% for positive queries
        let queries_present: Vec<u64> = inserted.iter().step_by(10).copied().take(n / 10).collect();

        // For negative queries, sample from the tail of the distribution
        // (items that exist in universe but are rare/never inserted)
        let inserted_set: std::collections::HashSet<u64> = inserted.iter().copied().collect();

        let mut queries_absent = Vec::new();
        let mut attempts = 0;
        while queries_absent.len() < n / 10 && attempts < n * 2 {
            let item = (cardinality as u64 / 2) + rng.random::<u64>() % (cardinality as u64 / 2);
            if !inserted_set.contains(&item) {
                queries_absent.push(item);
            }
            attempts += 1;
        }

        // Fill remaining with random items if needed
        while queries_absent.len() < n / 10 {
            let item: u64 = rng.random();
            if !inserted_set.contains(&item) {
                queries_absent.push(item);
            }
        }

        Dataset {
            inserted,
            queries_present,
            queries_absent,
        }
    }

    /// Generate a small dataset for quick tests
    pub fn small(seed: u64) -> Self {
        Self::uniform(1_000, seed)
    }

    /// Generate a medium dataset for standard benchmarks
    pub fn medium(seed: u64) -> Self {
        Self::uniform(100_000, seed)
    }

    /// Generate a large dataset for stress testing
    pub fn large(seed: u64) -> Self {
        Self::uniform(1_000_000, seed)
    }

    /// Get the actual cardinality (number of unique items)
    pub fn cardinality(&self) -> usize {
        let set: std::collections::HashSet<u64> = self.inserted.iter().copied().collect();
        set.len()
    }

    /// Get statistics about the dataset
    pub fn stats(&self) -> DatasetStats {
        let unique_items = self.cardinality();
        let total_items = self.inserted.len();
        let duplication_rate = 1.0 - (unique_items as f64 / total_items as f64);

        DatasetStats {
            total_items,
            unique_items,
            duplication_rate,
            queries_present: self.queries_present.len(),
            queries_absent: self.queries_absent.len(),
        }
    }
}

/// Statistics about a dataset
#[derive(Debug, Clone)]
pub struct DatasetStats {
    /// Total number of insertions
    pub total_items: usize,
    /// Number of unique items
    pub unique_items: usize,
    /// Fraction of duplicate insertions (0.0 = all unique, 0.9 = 90% duplicates)
    pub duplication_rate: f64,
    /// Number of positive query items
    pub queries_present: usize,
    /// Number of negative query items
    pub queries_absent: usize,
}

impl std::fmt::Display for DatasetStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Dataset Stats:\n  Total: {}\n  Unique: {}\n  Duplication: {:.1}%\n  Queries: {} present, {} absent",
            self.total_items,
            self.unique_items,
            self.duplication_rate * 100.0,
            self.queries_present,
            self.queries_absent
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_dataset() {
        let dataset = Dataset::uniform(10_000, 42);

        assert_eq!(dataset.inserted.len(), 10_000);
        assert_eq!(dataset.queries_present.len(), 1_000);
        assert_eq!(dataset.queries_absent.len(), 1_000);

        // Verify queries_present are actually in inserted
        let inserted_set: std::collections::HashSet<u64> =
            dataset.inserted.iter().copied().collect();

        for item in &dataset.queries_present {
            assert!(inserted_set.contains(item));
        }

        // Verify queries_absent are NOT in inserted
        for item in &dataset.queries_absent {
            assert!(!inserted_set.contains(item));
        }
    }

    #[test]
    fn test_zipfian_dataset() {
        let dataset = Dataset::zipfian(100_000, 10_000, 1.07, 42);

        assert_eq!(dataset.inserted.len(), 100_000);

        // Zipfian should have many duplicates
        let cardinality = dataset.cardinality();
        assert!(cardinality < 10_000);

        println!(
            "Zipfian dataset: {} unique out of {} total",
            cardinality, 100_000
        );
    }

    #[test]
    fn test_dataset_stats() {
        let dataset = Dataset::uniform(1_000, 42);
        let stats = dataset.stats();

        println!("{}", stats);

        assert_eq!(stats.total_items, 1_000);
        assert!(stats.unique_items <= 1_000);
        assert_eq!(stats.queries_present, 100);
        assert_eq!(stats.queries_absent, 100);
    }

    #[test]
    fn test_convenience_constructors() {
        let small = Dataset::small(42);
        assert_eq!(small.inserted.len(), 1_000);

        let medium = Dataset::medium(42);
        assert_eq!(medium.inserted.len(), 100_000);

        let large = Dataset::large(42);
        assert_eq!(large.inserted.len(), 1_000_000);
    }
}
