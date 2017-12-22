use std::fmt::Debug;

use self::Colour::*;

#[derive(Debug, PartialEq)]
pub struct RedBlackT<T: PartialOrd + PartialEq + Debug> {
	root: Option<Node<T>>,
	size: usize,
}

impl<T: PartialOrd + PartialEq + Debug> RedBlackT<T> {
	pub fn new() -> RedBlackT<T> {
		RedBlackT {
			root: None,
			size: 0
		}
	}

	pub fn check(ref rb: &RedBlackT<T>) -> bool {
		false
	}

	pub fn insert(&mut self, elem: T) {
		if self.root.is_none() {
			self.root = Some(Node::new(elem, Black));
		} else {
			// self.root.//insert(elem);
		}
	}

	pub fn remove(&mut self, elem: &T) -> Option<T> {
		None	
	}

	pub fn contains(&self, elem: &T) -> bool {
		false
	}

	pub fn size(&self) -> usize {
		self.size
	}

	pub fn is_empty(&self) -> bool {
		self.size == 0
	}
}

#[derive(Debug)]
struct Node<T: PartialOrd + PartialEq + Debug> {
	elem: T,
	colour: Colour
}

impl<T: PartialOrd + PartialEq + Debug> Node<T> {
	fn new(elem: T, colour: Colour) -> Node<T> {
		Node{ elem, colour }
	}

	fn insert(&mut self, elem: T) {

	}
}

impl<T> PartialEq for Node<T>
    where T: PartialOrd + PartialEq + Debug
{
	fn eq(&self, other: &Node<T>) -> bool {
		return self.elem== other.elem
	}
}

#[derive(Debug, PartialEq)]
enum Colour {
    Red,
    Black,
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn red_black_creates_with_new_factory() {
		let _: RedBlackT<i32> = RedBlackT::new();
	}

	#[test]
	fn red_black_is_empty_at_creation() {
		let rb: RedBlackT<i32> = RedBlackT::new();
		assert!(rb.is_empty());
	}

	#[test]
	fn red_black_returns_none_if_element_is_not_in_the_tree() {
		let mut rb: RedBlackT<i32> = RedBlackT::new();
		assert_eq!(None, rb.remove(&5));
	}

	#[test]
	fn red_black_can_insert() {
		let mut rb: RedBlackT<i32> = RedBlackT::new();
		rb.insert(4);
		assert!(rb.contains(&4));
		rb.insert(-10);
		assert!(rb.contains(&-10));
	}

	#[test]
	fn red_black_can_insert_duplicates() {
		let mut rb: RedBlackT<i32> = RedBlackT::new();
		rb.insert(5);
		rb.insert(5);
		assert_eq!(2, rb.size());
		rb.insert(5);
		assert_eq!(3, rb.size());
	}

	#[test]
	fn red_black_can_keep_track_of_size() {
		let mut rb: RedBlackT<i32> = RedBlackT::new();
		rb.insert(5);
		rb.insert(12);
		rb.insert(83);
		rb.insert(33);
		rb.insert(33);
		rb.insert(21);
		rb.insert(70);
		rb.insert(100);
		rb.insert(63);
		rb.insert(-9);
		rb.insert(23);
		assert_eq!(11, rb.size());
		rb.remove(&12);
		rb.remove(&83);
		rb.remove(&23);
		rb.remove(&-9);
		assert_eq!(7, rb.size());
		rb.insert(-9);
		rb.insert(23);
		assert_eq!(9, rb.size());
	}

	#[test]
	fn red_black_respects_property() {
		let mut rb: RedBlackT<i32> = RedBlackT::new();
		rb.insert(5);
		rb.insert(5);
		rb.insert(0);
		rb.insert(81);
		rb.insert(522);
		rb.insert(53);
		rb.insert(53);
		rb.insert(533);
		RedBlackT::check(&rb);
		rb.remove(&522);
		rb.remove(&-40);
		rb.remove(&53);
		rb.remove(&0);
		RedBlackT::check(&rb);
	}

	#[test]
	fn red_black_can_be_iterated_in_order() {
		let mut rb: RedBlackT<i32> = RedBlackT::new();
		// Creates a complex RBtree
		rb.insert(5);
		rb.insert(5);
		rb.insert(0);
		rb.insert(81);
		rb.insert(-25);
		rb.insert(1);
		rb.insert(3);
		rb.insert(-40);
		rb.insert(522);
		rb.insert(53);
		rb.insert(53);
		rb.insert(533);
		
		let expected = vec![-40, -25, 0, 1, 3, 5, 5, 53, 53, 81, 522, 533];

		for (elem, index) in rb.enumerator() {
			assert_eq!(expected.peek(index), elem);
		}
	}
}
