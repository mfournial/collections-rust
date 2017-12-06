pub mod priority_queue {

}

#[cfg(test)]
mod tests {
	use super::priority_queue;

	#[test]
	fn priority_queue_creates_with_new_factory() {
		priority_queue::new();
	}

	#[test]
	fn priority_queue_can_create_with_empty_macro() {
		pqueue!();
	}

	#[test]
	fn priority_queue_is_empty_with_new_factory() {
		let pq = priority_queue::new();
		assert!(pq.is_empty());
	}

	#[test]
	fn priority_queue_is_empty_with_macro() {
		let pq = pqueue!();
		assert!(pq.is_empty());
	}

	#[test]
	fn priority_queue_can_insert_with_factory() {
		let pq = priority_queue::new()
			.push(1)
			.push(5);
		assert_eq!(2, pq.size());
	}

	#[test]
	fn priority_queue_can_insert_with_macro() {
		let pq = pqueue!(1, 2, 3)
		assert_eq!(3, pq.size());
	}

	#[test]
	fn priority_queue_can_insert_duplicates() {
		let pq = pqueue!(1, 1, 1, 1);
		assert_eq!(4, pq.size());
	}

	#[test]
	fn priority_queue_can_be_equated() {
		let pq1 = pqueue!('c', 'd', 'p');
		let pq2 = pqueue!('c', 'd', 'p');
		assert_eq!(pq1, pq2);
	}

	#[test]
	fn priority_queue_can_retrieve_max() {
		let pq1 = pqueue!(1);
		assert_eq!(1, pq1.peek_max());
		let pq2 = pqueue!(5, 4, 3, 2, 1);
		assert_eq!(5, pq2.peek_max());
		let pq3 = pqueue!('1', '2', '3', '4', '5', '6', '7', '8');
		assert_eq!('8', pq3.peek_max());
	}

	#[test]
	fn priority_queue_can_be_iterated() {
		let pq = pqueue!(1, 6, 2, 8, 4, 3, 2, 10, 7);
        let actual = vec!();
		for elem in pq.iter() {
			actual.push(elem);
		}
		let expected = vec!(10, 8, 7, 6, 4, 3, 2, 2, 1);
		assert_eq!(expected, actual);
	}

	#[test]
	fn priority_queue_can_be_turned_into_a_vector() {
		let pq = pqueue!(1, 6, 2, 8, 4, 3, 2, 10, 7);
		let expected = vec!(10, 8, 7, 6, 4, 3, 2, 2, 1);
		assert_eq!(expected.as_vector(), pq);
	}

}