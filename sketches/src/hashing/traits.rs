pub trait Hasher64 {
    fn with_seed(seed: u64) -> Self
    where
        Self: Sized;

    fn hash(&self, bytes: &[u8]) -> u64;

    fn hash_with_seed(bytes: &[u8], seed: u64) -> u64
    where
        Self: Sized,
    {
        Self::with_seed(seed).hash(bytes)
    }
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
    pub fn prop_different_seeds<H, F>(
        seed1: u64,
        seed2: u64,
        data: Vec<u8>,
        constructor: F,
    ) -> TestResult
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

    /// Property: The seed parameter in hash_with_seed should vary the output
    /// For any hasher and two different seed parameters, outputs should differ
    pub fn prop_seed_parameter_varies<H>(seed1: u64, seed2: u64, data: Vec<u8>) -> TestResult
    where
        H: Hasher64,
    {
        if seed1 == seed2 || data.is_empty() {
            return TestResult::discard();
        }
        let hash1 = H::hash_with_seed(&data, seed1);
        let hash2 = H::hash_with_seed(&data, seed2);
        TestResult::from_bool(hash1 != hash2)
    }
}
