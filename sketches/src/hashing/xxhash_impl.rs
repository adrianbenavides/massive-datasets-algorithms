use super::Hasher64;
use xxhash_rust::xxh3::xxh3_64_with_seed;

#[derive(Clone)]
pub struct XXHasher {
    seed: u64,
}

impl Default for XXHasher {
    fn default() -> Self {
        Self { seed: 0 }
    }
}

impl XXHasher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_seed(seed: u64) -> Self {
        Self { seed }
    }
}

impl Hasher64 for XXHasher {
    fn hash(&self, bytes: &[u8]) -> u64 {
        xxh3_64_with_seed(bytes, self.seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hashing::traits::base_tests;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_xxhash_deterministic(seed: u64, data: Vec<u8>) -> bool {
        base_tests::prop_deterministic(seed, data, XXHasher::with_seed)
    }

    #[quickcheck]
    fn prop_xxhash_different_seeds(seed1: u64, seed2: u64, data: Vec<u8>) -> TestResult {
        base_tests::prop_different_seeds(seed1, seed2, data, XXHasher::with_seed)
    }
}
