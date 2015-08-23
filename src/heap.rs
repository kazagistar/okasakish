pub trait Heap<T: Ord + Clone> {
	fn empty() -> Self;
	fn is_empty(&self) -> bool;
	fn insert(&self, item: T) -> Self;
	fn merge(&self, other: &Self) -> Self;
	fn find_min(&self) -> Option<T>;
	fn delete_min(&self) -> Self;
}

#[cfg(test)]
pub mod properties {
	extern crate quickcheck;
	use self::quickcheck::{ quickcheck, Arbitrary };
	use std::fmt::Debug;
	use super::Heap;

	pub fn sorting<T: Ord + Clone + Debug + Arbitrary, H: Debug + Heap<T>>() {
		fn prop<T: Ord + Clone + Debug, H: Debug + Heap<T>>(mut input: Vec<T>) -> bool {
			let mut heap: H = Heap::empty();
			for item in input.iter() {
				heap = heap.insert(item.clone());
			}
			let mut output: Vec<T> = vec![];
			loop {
				if heap.is_empty() { break; }
				output.push(heap.find_min().unwrap());
				heap = heap.delete_min();
			}
			input.sort();
			input == output
		}
		quickcheck(prop::<T,H> as fn(Vec<T>) -> bool);
	}
}
