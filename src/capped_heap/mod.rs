use min_max_heap::MinMaxHeap;

#[cfg(test)]
mod tests;

/// A generic heap capped at a specific size
pub struct CappedHeap<T>
    where
        T: PartialOrd + Ord,
{
    pub cap: usize,
    pub heap: MinMaxHeap<T>,
}

impl<T> CappedHeap<T>
    where
        T: PartialOrd + Ord,
{
    /// Initializes a `CappedHeap`
    pub fn new(cap: usize) -> Self {
        let heap = MinMaxHeap::with_capacity(cap);
        CappedHeap { cap, heap }
    }

    /// Inserts an item into the heap if it fits, returning `true` if it was inserted
    pub fn insert(&mut self, item: T) -> bool {
        if self.heap.len() < self.cap {
            // Heap has not yet reached cap--safe to insert
            self.heap.push(item);
            true
        } else if self.cap > 0 && item > *self.heap.peek_min().unwrap() {
            // Heap is at cap but item has higher priority than current min--replace min
            self.heap.replace_min(item);
            true
        } else {
            // Heap is at cap and item has lower priority than everything in the heap--do nothing
            false
        }
    }
}
