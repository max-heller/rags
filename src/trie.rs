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

    /// Attempts to produce a reference to a key's node
    pub fn get_node<I, Q>(&self, key: I) -> Option<&Trie<K, V>>
    where
        I: IntoIterator<Item = Q>,
        K: From<Q>,
    {
        key.into_iter()
            .try_fold(self, |trie, fragment| trie.children.get(&K::from(fragment)))
    }

    /// Attempts to produce a reference to a key's value
    pub fn get<I, Q>(&self, key: I) -> Option<&V>
    where
        I: IntoIterator<Item = Q>,
        K: From<Q>,
    {
        self.get_node(key).map(|node| &node.value)
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
#[derive(Debug)]
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

impl<K, V> PartialEq for KeyValue<K, V>
where
    K: TrieKey,
    V: TrieValue + PartialEq + Eq + PartialOrd + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<K, V> Eq for KeyValue<K, V>
where
    K: TrieKey,
    V: TrieValue + PartialEq + Eq + PartialOrd + Ord,
{
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

#[test]
fn trie_construction() {
    let mut trie: Trie<String, u32> = Trie::new();
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

    // Leaf values
    assert_eq!(trie.get(vec!["ls", "-l", "/home"]), Some(&2));
    assert_eq!(trie.get(vec!["ls", "-l", "/dev"]), Some(&1));
    assert_eq!(trie.get(vec!["cd", "/dev"]), Some(&1));

    // Intermediate values
    assert_eq!(trie.get(vec!["ls", "-l"]), Some(&3));
    assert_eq!(trie.get(vec!["ls"]), Some(&4));
}
