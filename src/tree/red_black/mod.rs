use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct RedBlackT<K: PartialOrd + PartialEq + Debug, V: Debug + PartialEq> {
	root: Node<K, V>,
	size: usize,
}

#[derive(Debug)]
struct Node<K: PartialOrd + PartialEq + Debug, V: Debug + PartialEq> {
	key: K,
	value: V,
	colour: Colour
}

impl<K, V> PartialEq for Node<K, V>
    where K: PartialOrd + PartialEq + Debug, V: Debug + PartialEq
{
	fn eq(&self, other: &Node<K, V>) -> bool {
		return self.key == other.key && self.value == other.value;
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
		RedBlackT::check(rb);
		rb.remove(&522);
		rb.remove(&-40);
		rb.remove(&53);
		rb.remove(&0);
		RedBlackT::check(rb);
	}

	#[test]
	fn red_black_can_be_iterated_in_order() {
		let mut rb: RedBlackT<i32> = RedBlackT::new();
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
	}
}
