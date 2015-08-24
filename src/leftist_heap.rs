use std::sync::Arc;
use std::iter::FromIterator;
pub use super::heap::Heap;

#[derive(Debug)]
pub struct LeftistHeap<T> {
	head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
	rank: i32,
	elem: T,
	a: Link<T>,
	b: Link<T>,
}

fn link<T>(rank: i32, elem: T, a: Link<T>, b: Link<T>) -> Link<T> {
	Some(Arc::new(Node {
		rank: rank,
		elem: elem,
		a: a,
		b: b,
	}))
}

fn rank<T>(link: &Link<T>) -> i32 {
	match link.as_ref() {
		None => 0,
		Some(node) => node.rank,
	}
}

impl<T: Ord + Clone> Heap<T> for LeftistHeap<T> {
	/// O(1)
	fn empty() -> Self {
		LeftistHeap { head: None }
	}

	/// O(1)
	fn is_empty(&self) -> bool {
		self.head.is_none()
	}

	/// O(log(n))
	fn insert(&self, item: T) -> Self {
		self.merge(&LeftistHeap { head: link(1, item, None, None) })
	}


	/// O(log(n))
	fn merge(&self, other: &Self) -> Self {
		LeftistHeap { head: match (self.head.as_ref(), other.head.as_ref()) {
			(None, None) => None,
			(Some(h1), None) => Some(h1.clone()),
			(None, Some(h2)) => Some(h2.clone()),
			(Some(h1), Some(h2)) => {
				let (elem, a, b) = if h1.elem <= h2.elem {
					let wrapped = LeftistHeap { head: h1.b.clone() };
					(h1.elem.clone(), h1.a.clone(), (&wrapped).merge(other).head)
				} else {
					let wrapped = LeftistHeap { head: h2.b.clone() };
					(h2.elem.clone(), h2.a.clone(), self.merge(&wrapped).head)
				};
				let ra = rank(&a);
				let rb = rank(&b);
				if ra >= rb {
					link(rb + 1, elem, a, b)
				} else {
					link(ra + 1, elem, b, a)
				}
			}
		}}
	}

	/// O(1)
	fn find_min(&self) -> Option<T> {
		self.head.as_ref().map(|node| {
			node.elem.clone()
		})
	}

	/// O(log(n))
	fn delete_min(&self) -> Self {
		match self.head.as_ref() {
			None => Self::empty(),
			Some(node) => {
				let wrapped_a = LeftistHeap { head: node.a.clone() };
				let wrapped_b = LeftistHeap { head: node.b.clone() };
				wrapped_a.merge(&wrapped_b)
			}
		}
	}
}

pub struct IntoIter<T> {
	next: LeftistHeap<T>,
}

impl<T: Ord + Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		let item = self.next.find_min();
		self.next = self.next.delete_min();
		item
	}
}

impl<T: Ord + Clone> IntoIterator for LeftistHeap<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;
	fn into_iter(self) -> IntoIter<T> {
		IntoIter { next: self }
	}
}

impl<T: Ord + Clone> FromIterator<T> for LeftistHeap<T> {
	/// full iteration = O(n)
	fn from_iter<I>(iterator: I) -> Self where I: IntoIterator<Item=T> {
		let mut iter = iterator.into_iter();
		let mut stack: Vec<LeftistHeap<T>> = vec![];
		while let Some(item) = iter.next() {
			stack.push(LeftistHeap { head: link(1, item, None, None) });
			loop {
				// Only merge similar sized heaps
				let end = stack.len();
				if end < 2 { break; }
				if rank(&stack[end-2].head) <= rank(&stack[end-1].head) { break; }
				// Unwrap safe because above checks
				let a = stack.pop().unwrap();
				let b = stack.pop().unwrap();
				stack.push(a.merge(&b));
			}
		}
		// Merge remaining
		while stack.len() > 1 {
			// Unwrap safe because above check
			let a = stack.pop().unwrap();
			let b = stack.pop().unwrap();
			stack.push(a.merge(&b));
		}
		// Pop final merged heap if there was any items
		stack.pop().unwrap_or(Heap::empty())
	}
}

#[cfg(test)]
mod test {
	use super::{ LeftistHeap, Heap };
	use std::iter::FromIterator;

	#[test]
	fn basics() {
		let heap: LeftistHeap<i32> = Heap::empty();
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
	fn merge_and_iter() {
		let empty = LeftistHeap::<i32>::empty();
		let h1 = empty.insert(5).insert(1).insert(7).insert(3);
		let h2 = empty.insert(2).insert(6).insert(4);

		let merged = h1.merge(&h2);
		let ordered = Vec::<i32>::from_iter(merged);
		let sequence = Vec::<i32>::from_iter(1..8);
		assert_eq!(ordered, sequence);
	}

	#[test]
	fn from_iter() {
		let original = vec![5,1,7,3,2,6,4];
		let heap = LeftistHeap::<i32>::from_iter(original);

		let ordered = Vec::<i32>::from_iter(heap);
		let sequence = Vec::<i32>::from_iter(1..8);
		assert_eq!(ordered, sequence);
	}

	#[test]
	fn from_iter_empty() {
		let original = vec![];
		let heap = LeftistHeap::<i32>::from_iter(original);

		let ordered = Vec::<i32>::from_iter(heap);
		assert_eq!(ordered.len(), 0);
	}

	#[test]
	fn thread_safety() {
		fn is_send<T: Send>(){}
		is_send::<LeftistHeap<i32>>();

		fn is_sync<T: Sync>(){}
		is_sync::<LeftistHeap<i32>>();

		assert!(true);
	}

	use super::super::heap::properties;

	#[test]
	fn heap_quickchecks() {
		properties::sorting::<i32, LeftistHeap<i32>>();
	}
}
