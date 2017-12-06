#[derive(Debug)]
pub struct PriorityQueue<T: PartialOrd + PartialEq> {
	heap: Vec<T>,
	next_index: usize
}

impl<T: PartialOrd + PartialEq> PriorityQueue<T> {
	#[inline]
	pub fn new() -> PriorityQueue<T> {
		PriorityQueue {
			heap: Vec::new(),
			next_index: 0,
		}
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> PriorityQueue<T> {
        PriorityQueue {
            heap: Vec::with_capacity(capacity),
            next_index: 0,
        }
    }

	pub fn push(&mut self, elem: T) {

	}

	pub fn size(&self) -> usize {
		self.heap.len()
	}

	pub fn is_empty(&self) -> bool {
		self.heap.is_empty()
	}

	pub fn peek(&self) -> Option<&T> {
		if self.heap.is_empty() {
			return None;
		}
		Some(&self.heap[0])
	}

	pub fn pool(&mut self) -> Option<T> {
		// self.next_index -= 1;
		None
	}

	pub fn as_slice(&self) -> &[T] {
		self.heap.as_slice()
	}

	fn swap(&mut self, a: usize, b: usize) {
		self.heap.swap(a, b)
	}
}

fn parent(child: usize) -> usize {
	child - 1 / 2
}

fn right_ch(parent: usize) -> usize {
	parent * 2 + 2
}

fn left_ch(parent: usize) -> usize {
	parent * 2 + 1
}

impl<T: PartialOrd> Iterator for PriorityQueue<T> {
	type Item = T;

	fn next(&mut self) -> Option<T> {
		None
	}
}

impl<T: PartialEq + PartialOrd> PartialEq for PriorityQueue<T> {
	 fn eq(&self, other: &PriorityQueue<T>) -> bool {
	 	false
	 }
}

#[macro_export]
macro_rules! pqueue {
    ($elem:expr; $n:expr) => (
    	{
    		let mut temp_pq = PriorityQueue::new();
    		temp_pq.heap = vec![$elem; $n];
    		temp_pq.next_index = $n - 1;
    	}
    );
    ($($x:expr),*) => (
    	{
    		let mut temp_pq = PriorityQueue::new();
        	$(
        	    temp_pq.push($x);
        	)*
        	temp_pq
        }
        	// <[_]>::into_vec(box [$($x),*])
    );
    ( $($x: expr,)* ) => (pqueue![$($x),*])
    // ( $($x: expr),* ) => {
    //     {
    //         let mut temp_pq = PriorityQueue::new();
    //         $(
    //             temp_pq.push($x);
    //         )*
    //         temp_pq
    //     }
    // };
}


#[cfg(test)]
mod tests {
	// Ideas: from Vec, from slice, pqueue![1,2,3]
	use super::*;

	#[test]
	fn priority_queue_creates_with_new_factory() {
		let _: PriorityQueue<i32> = PriorityQueue::new();
	}

	#[test]
	fn priority_queue_can_create_with_empty_macro() {
		let _: PriorityQueue<i32> = pqueue!();
	}

	#[test]
	fn priority_queue_is_empty_with_new_factory() {
		let pq: PriorityQueue<i32> = PriorityQueue::new();
		assert!(pq.is_empty());
	}

	#[test]
	fn priority_queue_is_empty_with_macro() {
		let pq: PriorityQueue<i32> = pqueue!();
		assert!(pq.is_empty());
	}

	#[test]
	fn priority_queue_can_insert_with_factory() {
		let mut pq = PriorityQueue::new();
		pq.push(1);
		pq.push(5);
		assert_eq!(2, pq.size());
	}

	#[test]
	fn priority_queue_can_insert_with_macro() {
		let pq = pqueue!(1, 2, 3);
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
		let pq2 = pqueue!('c', 'p', 'd');
		assert_eq!(pq1, pq2);
	}

	#[test]
	fn priority_queue_can_peek() {
		let pq1 = pqueue!(1);
		assert_eq!(&1, pq1.peek().unwrap());
		let pq2 = pqueue!(5, 4, 3, 2, 1);
		assert_eq!(&5, pq2.peek().unwrap());
		let pq3 = pqueue!('1', '2', '3', '4', '5', '6', '7', '8');
		assert_eq!(&'8', pq3.peek().unwrap());
	}

	#[test]
	fn priority_queue_reorders_on_retrival_of_maximum() {
		let pq = pqueue!(1, -2, 32, -4, 5, 6, -90);
		pq.poll();
		pq.poll();
		pq.poll();
		assert_eq!(vec!(5, 1, -2, -4, -90).as_slice(), pq.as_slice());
	}

	#[test]
	fn priority_queue_can_be_iterated() {
		let pq = pqueue!(1, 6, 2, 8, 4, 3, 2, 10, 7);
        let mut actual = vec!();
		for elem in pq {
			actual.push(elem);
		}
		let expected = vec!(10, 8, 7, 6, 4, 3, 2, 2, 1);
		assert_eq!(expected, actual);
	}

	#[test]
	fn priority_queue_can_be_turned_into_a_vector() {
		let pq = pqueue!(1, 6, 2, 8, 4, 3, 2, 10, 7);
		let expected = vec!(10, 8, 7, 6, 4, 3, 2, 2, 1);
		assert_eq!(expected.as_slice(), pq.as_slice());
	}

}