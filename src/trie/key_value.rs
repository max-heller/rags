use std::cmp::Ordering;

use crate::trie::{TrieKey, TrieValue};

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