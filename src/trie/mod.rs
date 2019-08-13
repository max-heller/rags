use std::collections::HashMap;
use std::hash::Hash;

pub use key_value::KeyValue;

use crate::capped_heap::CappedHeap;

mod key_value;
#[cfg(test)]
mod tests;

/// Trait for types that can be used as keys in a `Trie`
pub trait TrieKey: Hash + Eq + Clone {}
impl<K: Hash + Eq + Clone> TrieKey for K {}

/// Trait for types that can be used as values in a `Trie`
pub trait TrieValue: Default {}
impl<V: Default> TrieValue for V {}

/// A generic trie with sequence-based keys
#[derive(Debug)]
pub struct Trie<K: TrieKey, V: TrieValue> {
    value: V,
    children: HashMap<K, Trie<K, V>>,
}

impl<K: TrieKey, V: TrieValue> Trie<K, V> {
    /// Initializes a new `Trie` with default value and no children
    pub fn new() -> Trie<K, V> {
        Trie {
            value: V::default(),
            children: HashMap::new(),
        }
    }

    /// Updates values along the path of a key using a function
    ///
    /// If nodes along the path are missing, they are initialized and then updated
    pub fn update_path<I, Q, F>(&mut self, key: I, f: F)
    where
        I: IntoIterator<Item = Q>,
        K: From<Q>,
        F: Fn(&V) -> V,
    {
        let mut target = key.into_iter().fold(self, |node, fragment| {
            // Update current node's value
            node.value = f(&node.value);
            // Find or create next node in path
            node.children
                .entry(K::from(fragment))
                .or_insert_with(Trie::new)
        });
        // Update final node's value
        target.value = f(&target.value);
    }
}

impl<K, V> Trie<K, V>
where
    K: TrieKey,
    V: TrieValue + PartialOrd + Ord,
{
    /// Produces a list of at most `n` key-value pairs sorted in descending order by value
    pub fn drain_top_items(mut self, n: usize) -> Vec<KeyValue<K, V>> {
        let mut heap = CappedHeap::new(n);
        for (fragment, node) in self.children.drain() {
            node.add_to_heap(vec![fragment], &mut heap);
        }
        heap.heap.into_vec_desc()
    }

    /// Adds the current node and its children to a `CappedHeap`
    fn add_to_heap(mut self, key: Vec<K>, heap: &mut CappedHeap<KeyValue<K, V>>) {
        // Create key-value pair for current node
        let item = KeyValue {
            key: key.to_owned(),
            value: self.value,
        };
        // Insert pair and all of its children if the former fit in the heap
        if heap.insert(item) {
            for (fragment, node) in self.children.drain() {
                // Build key for child
                let mut key = key.to_owned();
                key.push(fragment);

                // Add child to the heap
                node.add_to_heap(key, heap);
            }
        }
    }
}
