pub trait Hasher64 {
    fn hash(&self, bytes: &[u8]) -> u64;
}

#[cfg(test)]
pub mod base_tests {
    use crate::hashing::Hasher64;
    use quickcheck::TestResult;

    /// Property: A hasher with a fixed seed should produce deterministic output
    /// For any seed and data, hashing the same data twice produces identical results
    pub fn prop_deterministic<H, F>(seed: u64, data: Vec<u8>, constructor: F) -> bool
    where
        H: Hasher64,
        F: Fn(u64) -> H,
    {
        let hasher = constructor(seed);
        hasher.hash(&data) == hasher.hash(&data)
    }

    /// Property: Different seeds should produce different hashes
    /// For any two different seeds, the same data should (usually) produce different hashes
    pub fn prop_different_seeds<H, F>(seed1: u64, seed2: u64, data: Vec<u8>, constructor: F) -> TestResult
    where
        H: Hasher64,
        F: Fn(u64) -> H,
    {
        if seed1 == seed2 || data.is_empty() {
            return TestResult::discard();
        }
        let hasher1 = constructor(seed1);
        let hasher2 = constructor(seed2);
        TestResult::from_bool(hasher1.hash(&data) != hasher2.hash(&data))
    }
}
