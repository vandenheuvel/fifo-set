use std::collections::{HashSet, VecDeque};
use std::collections::vec_deque::Iter;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{Index, RangeBounds};

/// A FIFO queue with unique values.
#[derive(Clone, Default)]
pub struct FIFOSet<T> {
    deq: VecDeque<T>,
    set: HashSet<T>,
}

impl<T: Eq + Hash> FIFOSet<T> {
    pub fn new() -> Self {
        Self {
            deq: VecDeque::new(),
            set: HashSet::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            deq: VecDeque::with_capacity(capacity),
            set: HashSet::with_capacity(capacity),
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.deq.get(index)
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.deq.swap(i, j)
    }

    pub fn capacity(&self) -> usize {
        self.deq.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.deq.reserve(additional);
        self.set.reserve(additional);
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.deq.iter()
    }

    pub fn len(&self) -> usize {
        self.deq.len()
    }

    pub fn is_empty(&self) -> bool {
        self.deq.is_empty()
    }

    pub fn range<R: RangeBounds<usize>>(&self, range: R) -> Iter<'_, T> {
        self.deq.range(range)
    }

    pub fn clear(&mut self) {
        self.deq.clear();
        self.set.clear();
    }

    pub fn contains(&self, x: &T) -> bool {
        self.set.contains(x)
    }

    pub fn peek(&self) -> Option<&T> {
        self.deq.front()
    }

    /// Retrieve the item that has been in the queue longest.
    pub fn pop(&mut self) -> Option<T> {
        let next = self.deq.pop_front();

        if let Some(value) = next.as_ref() {
            self.set.remove(value);
        }

        next
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let removed = self.deq.remove(index);

        if let Some(value) = removed.as_ref() {
            self.set.remove(value);
        }

        removed
    }
}

impl<T: Copy + Eq + Hash> FIFOSet<T> {
    /// Add an item to the queue.
    pub fn push(&mut self, element: T) {
        if self.set.insert(element) {
            self.deq.push_back(element);
        }
    }
}

impl<T> Index<usize> for FIFOSet<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.deq.index(index)
    }
}

impl<A: Copy + Eq + Hash> FromIterator<A> for FIFOSet<A> {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
        let iterator = iter.into_iter();
        let (lower, _) = iterator.size_hint();
        let mut deq = FIFOSet::with_capacity(lower);
        deq.extend(iterator);
        deq
    }
}

impl<A: Copy + Eq + Hash> Extend<A> for FIFOSet<A> {
    fn extend<T: IntoIterator<Item=A>>(&mut self, iter: T) {
        for item in iter.into_iter() {
            self.push(item);
        }
    }
}
