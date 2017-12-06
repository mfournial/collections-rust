use std::fmt::Debug;

#[derive(Debug)]
pub struct PriorityQueue<T: PartialOrd + PartialEq + Debug> {
	heap: Vec<T>,
	next_index: usize
}

impl<T: PartialOrd + PartialEq + Debug> PriorityQueue<T> {
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
		let mut current = self.next_index;
		self.heap.push(elem);
		self.next_index += 1;

		while current != 0 && self.heap[current] > self.heap[parent(current)] {
            let current_parent = parent(current);
			self.swap(current, current_parent);
			current = current_parent;
		}
	}

	pub fn size(&self) -> usize {
		self.heap.len() // or could equally be self.next_index
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

	pub fn poll(&mut self) -> Option<T> {
		if self.is_empty() {
			return None;
		}

		let result = self.heap.remove(0);
		self.next_index -= 1;
		if self.next_index != 0 {
			self.siftdown(0);
		}

		Some(result)
	}

	pub fn as_slice(&self) -> &[T] {
		self.heap.as_slice()
	}

	fn swap(&mut self, a: usize, b: usize) {
		self.heap.swap(a, b)
	}

	fn siftdown(&mut self, start_index: usize) {
		let mut index = start_index;
		assert!(0 < index && index < self.next_index);

		while !self.is_leaf(index) {
			let left_ch = left_ch(index);
			let right_ch = right_ch(index);

			let max_ch_index = if right_ch < self.next_index && self.heap[left_ch] < self.heap[right_ch] {
				right_ch
			} else {
				left_ch
			};

			if self.heap[max_ch_index] < self.heap[index] {
				return
			}

			self.swap(max_ch_index, index);
			index = max_ch_index;
		}
	}

	fn is_leaf(&self, index: usize) -> bool {
		index >= self.next_index / 2 && index < self.next_index
	}
}

fn parent(child: usize) -> usize {
	(child - 1) / 2
}

fn right_ch(parent: usize) -> usize {
	parent * 2 + 2
}

fn left_ch(parent: usize) -> usize {
	parent * 2 + 1
}

impl<T: PartialOrd + Debug> Iterator for PriorityQueue<T> {
	type Item = T;

	fn next(&mut self) -> Option<T> {
		None
	}
}

impl<T: PartialEq + PartialOrd + Debug> PartialEq for PriorityQueue<T> {
	 fn eq(&self, other: &PriorityQueue<T>) -> bool {
	 	false
	 }
}

#[macro_export]
macro_rules! pqueue {
    () => (PriorityQueue::new());
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
	fn priority_queue_can_retrieve_largest() {
		let mut pq = PriorityQueue::new();
		pq.push(-4);
		pq.push(5);
		pq.push(-3);
		pq.push(8);
		pq.push(0);
		assert_eq!(&8, pq.peek().unwrap());
	}

	#[test]
	fn priority_queue_can_peek_with_macro() {
		let pq1 = pqueue!(1);
		assert_eq!(&1, pq1.peek().unwrap());
		let pq2 = pqueue!(-5, 4, 3, 2, 1);
		assert_eq!(&4, pq2.peek().unwrap());
		let pq3 = pqueue!('1', '2', '3', '4', '5', '6', '7', '8');
		assert_eq!(&'8', pq3.peek().unwrap());
	}

	#[test]
	fn priority_queue_reorders_on_retrival_of_maximum() {
		let mut pq = pqueue!(1, -2, 32, -4, 5, 6, -90);
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
	fn priority_queue_can_be_turned_into_a_slice() {
		let pq = pqueue!(1, 6, 2, 8, 4, 3, 2, 10, 7);
		let expected = vec!(10, 8, 7, 6, 4, 3, 2, 2, 1);
		assert_eq!(expected.as_slice(), pq.as_slice());
	}

}