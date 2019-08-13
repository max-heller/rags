use std::cmp::min;

use crate::trie::key_value::KeyValue;
use crate::trie::Trie;

impl Trie<&'static str, u32> {
    /// Attempts to produce a reference to a key's value
    pub fn get(&self, key: Vec<&'static str>) -> Option<&u32> {
        key.into_iter()
            .try_fold(self, |trie, fragment| trie.children.get(&fragment))
            .map(|node| &node.value)
    }
}

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