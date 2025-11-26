use super::Hasher64;
use ahash::RandomState;
use std::hash::{BuildHasher, Hasher};

#[derive(Clone)]
pub struct AHasher {
    state: RandomState,
}

impl Default for AHasher {
    fn default() -> Self {
        Self {
            state: RandomState::new(),
        }
    }
}

impl AHasher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            state: RandomState::with_seeds(seed, seed, seed, seed),
        }
    }
}

impl Hasher64 for AHasher {
    fn hash(&self, bytes: &[u8]) -> u64 {
        let mut hasher = self.state.build_hasher();
        hasher.write(bytes);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hashing::traits::base_tests;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_ahash_deterministic(seed: u64, data: Vec<u8>) -> bool {
        base_tests::prop_deterministic(seed, data, AHasher::with_seed)
    }

    #[quickcheck]
    fn prop_ahash_different_seeds(seed1: u64, seed2: u64, data: Vec<u8>) -> TestResult {
        base_tests::prop_different_seeds(seed1, seed2, data, AHasher::with_seed)
    }
}
