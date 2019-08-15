use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
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
    pub(crate) value: V,
    pub(crate) children: HashMap<K, Trie<K, V>>,
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
