use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

use min_max_heap::MinMaxHeap;

use crate::capped_heap::CappedHeap;

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

    /// Attempts to produce a reference to a key's value
    #[cfg(test)]
    pub fn get<I, Q>(&self, key: I) -> Option<&V>
    where
        I: IntoIterator<Item = Q>,
        K: From<Q>,
    {
        key.into_iter()
            .try_fold(self, |trie, fragment| trie.children.get(&K::from(fragment)))
            .map(|node| &node.value)
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

/// A key-value pair for a `Trie`
#[derive(Debug, PartialEq, Eq)]
pub struct KeyValue<K, V>
where
    K: TrieKey,
    V: TrieValue,
{
    pub key: Vec<K>,
    pub value: V,
}

impl<K, V> Ord for KeyValue<K, V>
where
    K: TrieKey,
    V: TrieValue + PartialEq + Eq + PartialOrd + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<K, V> PartialOrd for KeyValue<K, V>
where
    K: TrieKey,
    V: TrieValue + PartialEq + Eq + PartialOrd + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K, V> Trie<K, V>
where
    K: TrieKey,
    V: TrieValue + PartialEq + Eq + PartialOrd + Ord,
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

#[cfg(test)]
mod tests {
    use std::cmp::min;

    use super::*;

    fn init_trie() -> Trie<&'static str, u32> {
        let mut trie = Trie::new();
        let keys = vec![
            vec!["ls", "-l", "/home"],
            vec!["ls", "-l", "/home"],
            vec!["ls", "-l", "/dev"],
            vec!["cd", "/dev"],
            vec!["ls"],
        ];
        for key in keys {
            trie.update_path(key, |v| v + 1)
        }
        trie
    }

    /// Portion of sorted output that has deterministic order
    fn sorted() -> Vec<KeyValue<&'static str, u32>> {
        vec![
            KeyValue {
                key: vec!["ls"],
                value: 4,
            },
            KeyValue {
                key: vec!["ls", "-l"],
                value: 3,
            },
            KeyValue {
                key: vec!["ls", "-l", "/home"],
                value: 2,
            }
        ]
    }

    /// Portion of sorted output that might be in any order
    fn rest() -> Vec<KeyValue<&'static str, u32>> {
        vec![
            KeyValue {
                key: vec!["cd"],
                value: 1,
            },
            KeyValue {
                key: vec!["cd", "/dev"],
                value: 1,
            },
            KeyValue {
                key: vec!["ls", "-l", "/dev"],
                value: 1,
            }
        ]
    }

    fn assert_sorted(items: Vec<KeyValue<&'static str, u32>>) {
        let len_to_compare = min(items.len(), sorted().len());
        assert_eq!(items[..len_to_compare], sorted()[..len_to_compare]);
        if items.len() >= 6 {
            for extra in rest() {
                assert!(items.contains(&extra))
            }
        }
    }

    #[test]
    fn trie_construction() {
        let trie = init_trie();

        // Leaf values
        assert_eq!(trie.get(vec!["ls", "-l", "/home"]), Some(&2));
        assert_eq!(trie.get(vec!["ls", "-l", "/dev"]), Some(&1));
        assert_eq!(trie.get(vec!["cd", "/dev"]), Some(&1));

        // Intermediate values
        assert_eq!(trie.get(vec!["ls", "-l"]), Some(&3));
        assert_eq!(trie.get(vec!["ls"]), Some(&4));
    }

    #[test]
    fn trie_drain_none() {
        let trie = init_trie();
        assert_eq!(trie.drain_top_items(0), vec![]);
    }

    #[test]
    fn trie_drain_one() {
        let trie = init_trie();
        assert_eq!(trie.drain_top_items(1)[..], sorted()[..1]);
    }

    /// Drains the exact number of items in the trie
    #[test]
    fn trie_drain_all() {
        let trie = init_trie();
        let drained = trie.drain_top_items(6);
        assert_eq!(drained.len(), 6);
        assert_sorted(drained);
    }

    /// Attempts to drain more pairs than exist
    #[test]
    fn trie_drain_more() {
        let trie = init_trie();
        let drained = trie.drain_top_items(50);
        assert_eq!(drained.len(), 6);
        assert_sorted(drained);
    }
}
