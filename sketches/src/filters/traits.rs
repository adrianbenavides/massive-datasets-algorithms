use std::hash::Hash;

pub trait ApproximateMembershipQuery<T: Hash> {
    fn insert(&mut self, item: &T);
    fn contains(&self, item: &T) -> bool;
    fn false_positive_rate(&self) -> f64;
    fn capacity(&self) -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
