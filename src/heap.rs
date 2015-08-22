pub trait Heap<T: Ord + Clone> {
	fn empty() -> Self;
	fn is_empty(&self) -> bool;
	fn insert(&self, item: T) -> Self;
	fn merge(&self, other: &Self) -> Self;
	fn find_min(&self) -> Option<T>;
	fn delete_min(&self) -> Self;
}
