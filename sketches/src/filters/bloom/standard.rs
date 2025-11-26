use crate::filters::traits::ApproximateMembershipQuery;
use crate::hashing::Hasher64;
use bit_vec::BitVec;
use std::hash::Hash;
use std::marker::PhantomData;

/// A standard Bloom filter implementation.
/// Uses a single contiguous bit array and double hashing for generating multiple hash functions.
///
pub struct BloomFilter<T, H: Hasher64> {
    bit_array: BitVec,
    m: usize,     // Number of bits
    k: usize,     // Number of hash functions
    n: usize,     // Expected number of elements
    f: f64,       // Configured false positive rate
    count: usize, // Actual number of inserted items
    _phantom_data: PhantomData<T>,
    _phantom_hasher: PhantomData<H>,
}

impl<T, H: Hasher64> BloomFilter<T, H> {
    pub fn new(capacity: usize, false_positive_rate: f64) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");
        let m = Self::calculate_m(capacity, false_positive_rate);
        let k = Self::calculate_k(m, capacity);
        BloomFilter {
            bit_array: BitVec::from_elem(m, false),
            m,
            k,
            n: capacity,
            f: false_positive_rate,
            count: 0,
            _phantom_data: PhantomData,
            _phantom_hasher: PhantomData,
        }
    }

    fn calculate_m(n: usize, f: f64) -> usize {
        (-(n as f64) * f.ln() / (2f64.ln().powi(2))).ceil() as usize
    }

    fn calculate_k(m: usize, n: usize) -> usize {
        ((m as f64 / n as f64) * 2f64.ln()).ceil() as usize
    }

    /// Generates k hash positions for an item using double hashing technique.
    ///
    /// This method computes only 2 actual hash values and then uses arithmetic operations
    /// to simulate k independent hash functions. This is significantly more efficient than
    /// computing k separate hashes.
    ///
    /// # Double Hashing Formula
    /// For each i in 0..k: `h_i(x) = (h1(x) + i * h2(x)) mod m`
    ///
    /// # Performance
    /// - Cost: 2 hash computations + k arithmetic operations
    /// - Alternative cost: k hash computations
    /// - Arithmetic operations (add, multiply, modulo) are orders of magnitude faster than hashing
    fn hash_positions(&self, item: &T) -> impl Iterator<Item = usize> + '_
    where
        T: Hash,
    {
        // Compute two base hash values (this is where the actual hashing happens)
        let hash1 = H::hash_with_seed(&self.to_bytes(item), 0) as u32;
        let hash2 = H::hash_with_seed(&self.to_bytes(item), 1) as u32;

        // Generate k positions using only arithmetic on the two hash values
        // Double hashing: h_i(x) = (h1(x) + i*h2(x)) mod m
        (0..self.k).map(move |i| {
            let combined = hash1.wrapping_add((i as u32).wrapping_mul(hash2));
            (combined as usize) % self.m
        })
    }

    fn to_bytes(&self, item: &T) -> Vec<u8>
    where
        T: Hash,
    {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher as StdHasher;
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        hasher.finish().to_le_bytes().to_vec()
    }
}

impl<T: Hash, H: Hasher64> ApproximateMembershipQuery<T> for BloomFilter<T, H> {
    fn insert(&mut self, item: &T) {
        let positions: Vec<usize> = self.hash_positions(item).collect();
        for pos in positions {
            self.bit_array.set(pos, true);
        }
        self.count += 1;
    }

    fn contains(&self, item: &T) -> bool {
        self.hash_positions(item).all(|pos| self.bit_array[pos])
    }

    fn false_positive_rate(&self) -> f64 {
        self.f
    }

    fn capacity(&self) -> usize {
        self.n
    }

    fn len(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hashing::AHasher;

    #[test]
    fn test_calculate_m() {
        // For n=1000, f=0.01, m should be ~9585
        let m = BloomFilter::<u64, AHasher>::calculate_m(1000, 0.01);
        assert!((9500..=9600).contains(&m));
    }

    #[test]
    fn test_calculate_k() {
        let m = 9585;
        let n = 1000;
        let k = BloomFilter::<u64, AHasher>::calculate_k(m, n);
        assert_eq!(k, 7); // k ≈ 6.64 → 7
    }

    #[test]
    fn test_insert_and_lookup() {
        let mut bf = BloomFilter::<_, AHasher>::new(100, 0.01);
        bf.insert(&42u64);
        bf.insert(&123u64);

        assert!(bf.contains(&42u64));
        assert!(bf.contains(&123u64));
        assert_eq!(bf.len(), 2);
    }

    #[test]
    fn test_no_false_negatives() {
        let mut bf = BloomFilter::<_, AHasher>::new(100, 0.01);
        let items = vec![1, 2, 3, 42, 100, 255, 1000];

        for item in &items {
            bf.insert(item);
        }

        for item in &items {
            assert!(bf.contains(item), "False negative for {}", item);
        }
    }
}
