// MIT License Mayeul (Mike) Fournial <mayeul.fournial@outlook.com> - 2017

use std::fmt::Debug;

/// A priority queue implementation based on an unbounded max heap.
/// The elements are ordered by implementation of the PartialOrd trait with
/// the biggest element readable in `O(1)` time
///
/// # Examples
///
/// ```
/// extern crate collections_more;
/// use collections_more::queue::priority_queue::PriorityQueue;
/// // snip ..
/// # fn main() {
/// let mut pq: PriorityQueue<i32> = PriorityQueue::new();
/// pq.push(4);
/// pq.push(-55);
/// pq.push(9);
/// pq.push(0);
///
/// assert_eq!(4, pq.len()); // Length of the priority queue
/// assert_eq!(Some(9), pq.poll()); // Removes the biggest element
/// assert_eq!(3, pq.len()); // Length after polling the biggest element
///
/// // Polls max element of the priority queue on each iteration
/// // Equivalent to "Heapsort" but space inefficient
/// let mut sorted = Vec::with_capacity(pq.len());
/// for elem in pq {
///     sorted.push(elem)
/// }
/// assert_eq!(vec!(4, 0, -55), sorted);
/// # }
/// 
/// ```
///
/// # Macro and creation
/// 
/// For fast creation of priority_queues, one can use the macro that behaves
/// like the `vec` macro (e.g. `let mut pq = pqueue!(3, 2, 7, -3)`, 
/// `let pq = pqueue!['c', 'h', 'a', 'r']`).  
/// We do not provide a `unsafe fn from_raw_part(ptr: *mut T, length: usize,
/// capacity: usize) -> PriorityQueue<T>) builder as it is too unsafe to do so.
///
/// 
/// # Slicing
/// 
/// It is possible to retrieve the priority queue as a slice. However it'll be
/// in a heap order, not consecutive natural ordering of the elements.
///
#[derive(Debug, PartialEq)]
pub struct PriorityQueue<T: PartialOrd + PartialEq + Debug> {
	heap: Vec<T>,
	next_index: usize
}

impl<T: PartialOrd + PartialEq + Debug> PriorityQueue<T> {
    /// Constructs a new, empty `PriorityQueue<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate collections_more;
    /// # use collections_more::queue::priority_queue::PriorityQueue;
    /// # fn main() {
    /// # #[allow(unused_mut)]
    /// let mut pq: PriorityQueue<i32> = PriorityQueue::new();
    /// # }
    /// ```
	#[inline]
	pub fn new() -> PriorityQueue<T> {
		PriorityQueue {
			heap: Vec::new(),
			next_index: 0,
		}
	}

    /// Constructs a new, empty `PriorityQueue<T>` with the specified capacity.
    ///
    /// The priority queue will be able to hold exactly `capacity` 
    /// elements without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate collections_more;
    /// # use collections_more::queue::priority_queue::PriorityQueue;
    /// # fn main() {
    /// let mut pq = PriorityQueue::with_capacity(10);
    ///
    /// // The pqueue is empty, even though it has capacity of 10
    /// assert_eq!(pq.len(), 0);
    ///
    /// // These are all done without reallocating...
    /// for i in 0..10 {
    ///     pq.push(i);
    /// }
    ///
    /// // ...but this may make the pqueue to reallocate
    /// pq.push(11);
    /// # }
    /// ```
	#[inline]
	pub fn with_capacity(capacity: usize) -> PriorityQueue<T> {
        PriorityQueue {
            heap: Vec::with_capacity(capacity),
            next_index: 0,
        }
    }

    /// Add an element to the priority queue while keeping the most desired
    /// element accessible in the same time
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate collections_more;
    /// # use collections_more::queue::priority_queue::PriorityQueue;
    /// # fn main() {
    /// let mut pq = PriorityQueue::with_capacity(2);
    /// pq.push(2);
    /// pq.push(6);
    /// assert_eq!(2, pq.len());        // length of the pqueue
    /// assert_eq!(Some(6), pq.poll()); // max element of pqueue
    /// # }
    /// ```
	pub fn push(&mut self, elem: T) {
		let mut current = self.next_index;
		self.heap.push(elem);
		self.next_index += 1;

		while current != 0 && self.heap[current] > self.heap[parent(current)] {
			self.swap(current, parent(current));
			current = parent(current);
		}
	}

    /// Returns the number of elements in the queue
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate collections_more;
    /// # use collections_more::queue::priority_queue::PriorityQueue;
    /// # fn main() {
    /// let mut pq = PriorityQueue::with_capacity(2);
    /// assert_eq!(0, pq.len()); // no elements
    /// pq.push(2);
    /// assert_eq!(1, pq.len()); // one elements
    /// pq.push(6);
    /// assert_eq!(2, pq.len());
    /// pq.poll(); 
    /// assert_eq!(1, pq.len()); // one elements again
    /// # }
    /// ```
    #[inline]
	pub fn len(&self) -> usize {
		self.heap.len() // or could equally be self.next_index
	}

    /// Returns true if there is no element in the queue.  
    /// Equivalent to `pq.len() == 0`
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate collections_more;
    /// # use collections_more::queue::priority_queue::PriorityQueue;
    /// # fn main() {
    /// let mut pq = PriorityQueue::with_capacity(2);
    /// assert!(pq.is_empty()); // no elements
    /// pq.push(2);
    /// assert!(!pq.is_empty()); // one elements
    /// pq.poll(); 
    /// assert!(pq.is_empty()); // no element again
    /// # }
    /// ```
    #[inline]
	pub fn is_empty(&self) -> bool {
		self.heap.is_empty()
	}

    /// Returns a borrow to the biggest element of the queue (O(1) time).  
    /// **returns `None` if queue is empty**
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate collections_more;
    /// # use collections_more::queue::priority_queue::PriorityQueue;
    /// # fn main() {
    /// let mut pq = PriorityQueue::with_capacity(2);
    /// pq.push(2);
    /// pq.push(6);
    /// assert_eq!(Some(&6), pq.peek());
    /// # }
    /// ```
	pub fn peek(&self) -> Option<&T> {
		if self.heap.is_empty() {
			return None;
		}
		Some(&self.heap[0])
	}

    /// Retrieves the biggest element of the queue, therefore deleting it from
    /// the queue.  
    /// **returns `None` if queue is empty**
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate collections_more;
    /// # use collections_more::queue::priority_queue::PriorityQueue;
    /// # fn main() {
    /// let mut pq = PriorityQueue::with_capacity(2);
    /// pq.push(2);
    /// pq.push(6);
    /// assert_eq!(Some(6), pq.poll());
    /// assert_eq!(1, pq.len());
    /// # }
    /// ```
	pub fn poll(&mut self) -> Option<T> {
		if self.is_empty() {
			return None;
		}

		self.next_index -= 1;
        self.heap.swap(0, self.next_index);
		if self.next_index != 0 {
			self.siftdown(0);
		}

		self.heap.pop()
	}

    /// Extracts a slice containing the entire priority queue.
    /// *Note the order of the slice will be a heap order, not sorted*
	pub fn as_slice(&self) -> &[T] {
		self.heap.as_slice()
	}

    #[inline]
	fn swap(&mut self, a: usize, b: usize) {
		self.heap.swap(a, b)
	}

	fn siftdown(&mut self, start_index: usize) {
		let mut index = start_index;

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
		self.poll()
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
    );
    ( $($x: expr,)* ) => (pqueue![$($x),*])
}


#[cfg(test)]
mod tests {
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
    fn priority_queue_returns_none_if_empty() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        assert_eq!(None, pq.poll());
        pq.push(4);
        pq.poll();
        assert_eq!(None, pq.poll());
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
		assert_eq!(2, pq.len());
	}

	#[test]
	fn priority_queue_can_insert_with_macro() {
		let pq = pqueue!(1, 2, 3);
		assert_eq!(3, pq.len());
	}

	#[test]
	fn priority_queue_can_insert_duplicates() {
		let pq = pqueue!(1, 1, 1, 1);
		assert_eq!(4, pq.len());
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
		let mut pq = pqueue![1, -2, 32, -4, 5, 6, -90];
		pq.poll();
		pq.poll();
		pq.poll();
        pq.poll();
		assert_eq!(vec!(-2, -4, -90).as_slice(), pq.as_slice());
	}

	#[test]
	fn priority_queue_can_be_iterated_in_order() {
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
		let expected = vec!(10, 8, 3, 7, 4, 2, 2, 1, 6);
		assert_eq!(expected.as_slice(), pq.as_slice());
	}

    #[test]
    fn priority_queue_works_with_heap_elements() {
        let pq = pqueue!(Box::new(3), Box::new(-2), Box::new(-9), Box::new(-2),
            Box::new(2), Box::new(3), Box::new(99), Box::new(14), Box::new(5));
        let expected = vec!(Box::new(99), Box::new(14), Box::new(5), Box::new(3),
            Box::new(3), Box::new(2), Box::new(-2), Box::new(-2), Box::new(-9));
        let mut actual = Vec::with_capacity(9);
        for elem in pq {
            actual.push(elem);
        }
        assert_eq!(expected, actual);
    }
}