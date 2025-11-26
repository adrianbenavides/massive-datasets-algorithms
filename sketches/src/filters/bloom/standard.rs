use crate::filters::traits::ApproximateMembershipQuery;
use crate::hashing::Hasher64;
use bit_vec::BitVec;
use std::hash::Hash;
use std::marker::PhantomData;

pub struct BloomFilter<T, H: Hasher64> {
    bit_array: BitVec,
    m: usize,     // Number of bits
    k: usize,     // Number of hash functions
    n: usize,     // Expected number of elements
    f: f64,       // Configured false positive rate
    count: usize, // Actual number of inserted items
    hasher: H,
    _phantom: PhantomData<T>,
}

impl<T, H: Hasher64> BloomFilter<T, H> {
    pub fn new(capacity: usize, false_positive_rate: f64, hasher: H) -> Self {
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
            hasher,
            _phantom: PhantomData,
        }
    }

    fn calculate_m(n: usize, f: f64) -> usize {
        (-(n as f64) * f.ln() / (2f64.ln().powi(2))).ceil() as usize
    }

    fn calculate_k(m: usize, n: usize) -> usize {
        ((m as f64 / n as f64) * 2f64.ln()).ceil() as usize
    }

    fn hash_positions(&self, item: &T) -> impl Iterator<Item = usize> + '_
    where
        T: Hash,
    {
        let hash1 = self.hasher.hash(&self.to_bytes(item)) as u32;
        let hash2 = H::hash_with_seed(&self.to_bytes(item), 1) as u32;

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
        assert!(m >= 9500 && m <= 9600);
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
        let mut bf = BloomFilter::new(100, 0.01, AHasher::default());
        bf.insert(&42u64);
        bf.insert(&123u64);

        assert!(bf.contains(&42u64));
        assert!(bf.contains(&123u64));
        assert_eq!(bf.len(), 2);
    }

    #[test]
    fn test_no_false_negatives() {
        let mut bf = BloomFilter::new(100, 0.01, AHasher::default());
        let items = vec![1, 2, 3, 42, 100, 255, 1000];

        for item in &items {
            bf.insert(item);
        }

        for item in &items {
            assert!(bf.contains(item), "False negative for {}", item);
        }
    }
}
