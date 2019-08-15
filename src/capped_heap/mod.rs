use min_max_heap::MinMaxHeap;

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
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

    /// Inserts an item into the heap if there is space for it or it ranks higher than current min
    pub fn insert(&mut self, item: T) {
        if self.heap.len() < self.cap {
            // Heap has not yet reached cap--safe to insert
            self.heap.push(item);
        } else if self.cap > 0 && item > *self.heap.peek_min().unwrap() {
            // Heap is at cap but item has higher priority than current min--replace min
            self.heap.replace_min(item);
        }
    }
}
