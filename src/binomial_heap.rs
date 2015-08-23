use std::sync::Arc;
use std::cmp;
use std::iter;
pub use super::heap::Heap;

#[derive(Debug)]
pub struct BinomialHeap<T> {
	trees: Vec<Option<Link<T>>>,
}

type Link<T> = Arc<Node<T>>;

#[derive(Clone, Hash, Debug)]
struct Node<T> {
	elem: T,
	children: Vec<Link<T>>,
}

fn link<T: Ord + Clone>(a: &Link<T>, b: &Link<T>) -> Link<T> {
	let (smaller, bigger) = if a.elem < b.elem { (a, b) } else { (b, a) };
	let mut new_children = Vec::with_capacity(smaller.children.len() + 1);
	new_children.extend(smaller.children.iter().cloned());
	new_children.push(bigger.clone());
	Arc::new(Node { elem: smaller.elem.clone(), children: new_children })
}

impl <T: Ord + Clone> Heap<T> for BinomialHeap<T> {
	fn empty() -> Self {
		BinomialHeap { trees: Vec::new() }
	}

	fn is_empty(&self) -> bool {
		self.trees.is_empty()
	}

	fn insert(&self, item: T) -> Self {
		BinomialHeap { trees: vec![
			Some(Arc::new(Node { elem: item, children: Vec::new()}))
		]}.merge(self)
	}

	fn merge<'a>(&self, other: &Self) -> Self {
		let cap = cmp::max(self.trees.len(), other.trees.len()) + 1;
		let mut result: Vec<Option<Link<T>>> = Vec::with_capacity(cap);
		let mut c: Option<Link<T>> = None;

		// Create infinite interator of Option<&Link<T>>
		let iter_self = self.trees.iter().map(Option::as_ref).chain(iter::repeat(None));
		let iter_other = other.trees.iter().map(Option::as_ref).chain(iter::repeat(None));
		// Link trees of equal rank, carrying the added trees up a rank
		for ((a, b), _) in iter_self.zip(iter_other).zip(0..cap) {
			result.push( match (a, b, c.take()) {
				(None, None, None) => None,
				(Some(x), None, None) => Some(x.clone()),
				(None, Some(y), None) => Some(y.clone()),
				(None, None, Some(z)) => Some(z.clone()),
				(Some(x), Some(y), None) => { c = Some(link(x,y)); None }
				(Some(x), None, Some(z)) => { c = Some(link(x,&z)); None }
				(None, Some(y), Some(z)) => { c = Some(link(y,&z)); None }
				(Some(x), Some(y), Some(z)) => { c = Some(link(x,y)); Some(z) }
			});
		}
		if let Some(&None) = result.last() {
			result.pop();
		}
		BinomialHeap { trees: result }
	}

	fn find_min(&self) -> Option<T> {
		match min_index(&self.trees) {
			None => None,
			Some(index) => Some(self.trees[index].as_ref().unwrap().elem.clone())
		}
	}

	fn delete_min(&self) -> Self {
		match min_index(&self.trees) {
			None => Heap::empty(),
			Some(index) => {
				let mut old_trees = self.trees.clone();
				let taken = old_trees[index].take().unwrap();
				let old = BinomialHeap { trees: old_trees };
				old.merge(&BinomialHeap { trees: taken.children.iter().cloned().map(Some).collect() })
			}
		}
	}
}

// Utility function to find the smallest index in the heap
fn min_index<T: Ord>(vec: &Vec<Option<Link<T>>>) -> Option<usize> {
	let mut i = vec.iter()                                           // Iterator<&Option<Link<T>>>
	               .enumerate()                                      // Iterator<(usize, &Option<Link<T>>)
	               .map(|(i, x)| x.as_ref().map(|v| (i, &(v.elem)))) // Iterator<Option<(usize, &T)>>
	               .filter(Option::is_some).map(Option::unwrap);     // Iterator<(usize, &T)>

	// Grab first item if it exists
	let first = i.next();
	let (ix, init) = match first {
		None => return None,
		Some(x) => x,
	};

	// Compare all the values, find the smallest, and return its index
	Some(i.fold((ix, init), |(ai, a), (bi, b)| if &a < &b { (ai, a) } else { (bi, b) }).0)
}

#[cfg(test)]
mod test {
	use super::{ BinomialHeap, Heap };

	#[test]
	fn basics() {
		let heap: BinomialHeap<i32> = Heap::empty();
		assert_eq!(heap.find_min(), None);

		let heap = heap.insert(2).insert(1).insert(3);
		assert_eq!(heap.find_min(), Some(1));

		let heap = heap.delete_min();
		assert_eq!(heap.find_min(), Some(2));

		let heap = heap.delete_min();
		assert_eq!(heap.find_min(), Some(3));

		let heap = heap.delete_min();
		assert_eq!(heap.find_min(), None);

		let heap = heap.delete_min();
		assert_eq!(heap.find_min(), None);
	}

	#[test]
	fn thread_safety() {
		fn is_send<T: Send>(){}
		is_send::<BinomialHeap<i32>>();

		fn is_sync<T: Sync>(){}
		is_sync::<BinomialHeap<i32>>();

		assert!(true);
	}
}
