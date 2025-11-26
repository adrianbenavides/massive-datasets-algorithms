use super::Hasher64;
use murmur3::murmur3_x64_128;
use std::io::Cursor;

#[derive(Clone, Default)]
pub struct Murmur3Hasher {
    seed: u32,
}

impl Murmur3Hasher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_seed(seed: u64) -> Self {
        Self { seed: seed as u32 }
    }
}

impl Hasher64 for Murmur3Hasher {
    fn with_seed(seed: u64) -> Self
    where
        Self: Sized,
    {
        Self { seed: seed as u32 }
    }

    fn hash(&self, bytes: &[u8]) -> u64 {
        let mut reader = Cursor::new(bytes);
        let hash128 = murmur3_x64_128(&mut reader, self.seed)
            .expect("murmur3 hash should not fail on in-memory data");
        // Take the lower 64 bits of the 128-bit hash
        hash128 as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hashing::traits::base_tests;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_murmur3_deterministic(seed: u64, data: Vec<u8>) -> bool {
        base_tests::prop_deterministic(seed, data, Murmur3Hasher::with_seed)
    }

    #[quickcheck]
    fn prop_murmur3_different_seeds(seed1: u64, seed2: u64, data: Vec<u8>) -> TestResult {
        base_tests::prop_different_seeds(seed1, seed2, data, Murmur3Hasher::with_seed)
    }

    #[quickcheck]
    fn prop_murmur3_seed_parameter_varies(param1: u64, param2: u64, data: Vec<u8>) -> TestResult {
        base_tests::prop_seed_parameter_varies::<Murmur3Hasher>(param1, param2, data)
    }
}
