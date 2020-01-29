#![allow(dead_code)]
use crossbeam::epoch::{pin, Atomic, Guard, Owned, Shared};
use crossbeam::utils::CachePadded;
use std::fmt::{Debug, Display};
use std::sync::atomic::Ordering;

/// This queue orders elements FIFO (first-in-first-out).
/// The <em>head</em> of the queue is that element that has been on the
/// queue the longest time.
/// The <em>tail</em> of the queue is that element that has been on the
/// queue the shortest time. New elements
/// are inserted at the tail of the queue, and the queue retrieval
/// operations obtain elements at the head of the queue.
/// A {@code ConcurrentLinkedQueue} is an appropriate choice when
/// many threads will share access to a common collection.
/// Like most other concurrent collection implementations, this class
/// does not permit the use of {@code null} elements.
///
/// <p>This implementation employs an efficient <em>non-blocking</em>
/// algorithm based on one described in
/// <a href="http://www.cs.rochester.edu/~scott/papers/1996_PODC_queues.pdf">
/// Simple, Fast, and Practical Non-Blocking and Blocking Concurrent Queue
/// Algorithms</a> by Maged M. Michael and Michael L. Scott.
///
/// <p>Iterators are <i>weakly consistent</i>, returning elements
/// reflecting the state of the queue at some point at or since the
/// creation of the iterator.  They do <em>not</em> throw {@link
/// java.util.ConcurrentModificationException}, and may proceed concurrently
/// with other operations.  Elements contained in the queue since the creation
/// of the iterator will be returned exactly once.
///
/// <p>Beware that, unlike in most collections, the {@code size} method
/// is <em>NOT</em> a constant-time operation. Because of the
/// asynchronous nature of these queues, determining the current number
/// of elements requires a traversal of the elements, and so may report
/// inaccurate results if this collection is modified during traversal.
///
/// <p>Bulk operations that add, remove, or examine multiple elements,
/// such as {@link #addAll}, {@link #removeIf} or {@link #forEach},
/// are <em>not</em> guaranteed to be performed atomically.
/// For example, a {@code forEach} traversal concurrent with an {@code
/// addAll} operation might observe only some of the added elements.
///
/// <p>This class and its iterator implement all of the <em>optional</em>
/// methods of the {@link Queue} and {@link Iterator} interfaces.
///
/// <p>Memory consistency effects: As with other concurrent
/// collections, actions in a thread prior to placing an object into a
/// {@code ConcurrentLinkedQueue}
/// <a href="package-summary.html#MemoryVisibility"><i>happen-before</i></a>
/// actions subsequent to the access or removal of that element from
/// the {@code ConcurrentLinkedQueue} in another thread.
///
/// <p>This class is a member of the
/// <a href="{@docRoot}/java.base/java/util/package-summary.html#CollectionsFramework">
/// Java Collections Framework</a>.
///
/// @since 1.5
/// @author Doug Lea
/// @param <E> the type of elements held in this queue

pub fn new_atomic_null() -> Atomic<()> {
    Atomic::null()
}

pub(crate) struct Node<T> {
    pub(crate) item: T,
    pub(crate) next: Atomic<Node<T>>,
}

impl<T> Node<T>
where
    T: Eq + Sized + Default + Display + Debug,
{
    pub(crate) fn new_empty() -> Node<T> {
        Node {
            item: T::default(),
            next: Atomic::null(),
        }
    }
    pub(crate) fn new(elem: T) -> Node<T> {
        Node {
            item: elem,
            next: Atomic::null(),
        }
    }
}

pub struct SelkirkLinkedQueue<T> {
    // A node from which the first live (non-deleted) node (if any)
    // can be reached in O(1) time.
    // Invariants:
    // - all live nodes are reachable from head via succ()
    // - head != null
    // - (tmp = head).next != tmp || tmp != head
    // Non-invariants:
    // - head.item may or may not be null.
    // - it is permitted for tail to lag behind head, that is, for tail
    //   to not be reachable from head!
    head: CachePadded<Atomic<Node<T>>>,

    // A node from which the last node on list (that is, the unique
    // node with node.next == null) can be reached in O(1) time.
    // Invariants:
    // - the last node is always reachable from tail via succ()
    // - tail != null
    // Non-invariants:
    // - tail.item may or may not be null.
    // - it is permitted for tail to lag behind head, that is, for tail
    //   to not be reachable from head!
    // - tail.next may or may not be self-linked.
    tail: CachePadded<Atomic<Node<T>>>,
}

unsafe impl<T> Send for SelkirkLinkedQueue<T> {}
unsafe impl<T> Sync for SelkirkLinkedQueue<T> {}

impl<T> SelkirkLinkedQueue<T>
where
    T: Eq + Sized + Default + Debug + Display,
{
    pub fn new() -> SelkirkLinkedQueue<T> {
        let head = CachePadded::new(Atomic::new(Node::<T>::new_empty()));
        let tail = head.clone();
        SelkirkLinkedQueue { head, tail }
    }
    pub fn add(&self, elem: T) {
        self.offer(elem)
    }

    // Returns the successor of p, or the head node if p.next has been
    // linked to self, which will only be true if traversing with a
    // stale pointer that is now off the list.
    fn succ<'g>(p: Atomic<Node<T>>, guard: &'g Guard) -> Atomic<Node<T>> {
        //        let current_node = p.load(Ordering::SeqCst, guard);
        //        let current_item = current_node.item.load(Ordering::SeqCst,guard);
        //        let next = p.next.load(Ordering::SeqCst, guard);
        //        if (p == next) {}
        return Atomic::null();
    }

    fn offer(&self, elem: T) {
        //        let guard = pin();
        //        let new_node = Node::new(elem);
        //        let mut p = t;
        //        loop {
        //            let t = &self.tail.load(Ordering::SeqCst, &guard);
        //            let q = p.load(Ordering::SeqCst, &guard);
        //            if q.is_null() {}
        //        }
    }

    pub fn enqueue(&self, elem: T) {
        let new_node = Node::new(elem);
        let guard = &pin();
        // Enqueue is done. Try to swing Tail to the inserted node
        // CAS(&Q–>Tail, tail, <node, tail.count+1>)
        let old_tail_node = self.tail.compare_and_set(
            self.tail.load(Ordering::SeqCst, guard),
            Owned::new(new_node),
            Ordering::AcqRel,
            guard,
        );
    }

    pub fn print_last(&self) {
        let guard = &pin();
        if let Some(tail_node) = unsafe { self.tail.load(Ordering::SeqCst, guard).as_ref() } {
            println!("tail_node: {}", tail_node.item)
        } else {
            println!("Fuck?");
        }
    }

    pub fn dequeue(&self) -> Result<T, ()> {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_null() {
        let guard = pin();
        assert!(new_atomic_null().load(Ordering::SeqCst, &guard).is_null());
    }

    #[test]
    fn enqueue_some() {
        use crossbeam::thread;
        let queue = SelkirkLinkedQueue::<i32>::new();
        let scope = thread::scope(|s| {
            s.spawn(|_| {
                queue.print_last();
                for i in 1..1000 {
                    queue.enqueue(i);
                }
                queue.print_last();
            });
            s.spawn(|_| {
                queue.print_last();
                for i in 1000..2000 {
                    queue.enqueue(i);
                }
                queue.print_last();
            });
        });
        queue.print_last();
        scope.unwrap();
    }
}
