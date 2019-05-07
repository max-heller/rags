use min_max_heap::MinMaxHeap;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

pub trait TrieKey: Hash + Eq + Clone {}
impl<K: Hash + Eq + Clone> TrieKey for K {}
pub trait TrieValue: Default {}
impl<V: Default> TrieValue for V {}

#[derive(Debug)]
pub struct Trie<K: TrieKey, V: TrieValue> {
    value: V,
    children: HashMap<K, Trie<K, V>>,
}

impl<K: TrieKey, V: TrieValue> Trie<K, V> {
    pub fn new() -> Trie<K, V> {
        Trie {
            value: V::default(),
            children: HashMap::new(),
        }
    }

    pub fn insert<I, Q>(&mut self, key: I, value: V)
    where
        I: IntoIterator<Item = Q>,
        K: From<Q>,
    {
        let mut target = key.into_iter().fold(self, |node, fragment| {
            node.children
                .entry(K::from(fragment))
                .or_insert_with(Trie::new)
        });
        target.value = value;
    }

    pub fn get_node<I, Q>(&self, key: I) -> Option<&Trie<K, V>>
    where
        I: IntoIterator<Item = Q>,
        K: From<Q>,
    {
        key.into_iter()
            .try_fold(self, |trie, fragment| trie.children.get(&K::from(fragment)))
    }

    pub fn get<I, Q>(&self, key: I) -> Option<&V>
    where
        I: IntoIterator<Item = Q>,
        K: From<Q>,
    {
        self.get_node(key).map(|node| &node.value)
    }

    pub fn update_path<I, Q, F>(&mut self, key: I, f: F)
    where
        I: IntoIterator<Item = Q>,
        K: From<Q>,
        F: Fn(&V) -> V,
    {
        let mut target = key.into_iter().fold(self, |node, fragment| {
            node.value = f(&node.value);
            node.children
                .entry(K::from(fragment))
                .or_insert_with(Trie::new)
        });
        target.value = f(&target.value);
    }
}

impl<K, V, S, I, Q> From<S> for Trie<K, V>
where
    K: TrieKey + From<Q>,
    V: TrieValue,
    S: IntoIterator<Item = (I, V)>,
    I: IntoIterator<Item = Q>,
{
    fn from(source: S) -> Self {
        let mut trie = Trie::new();
        for (key, value) in source {
            trie.insert(key, value);
        }
        trie
    }
}

pub struct SizedHeap<T>
where
    T: PartialOrd + Ord,
{
    size: usize,
    heap: MinMaxHeap<T>,
}

impl<T> SizedHeap<T>
where
    T: PartialOrd + Ord,
{
    pub fn new(n: usize) -> Self {
        SizedHeap {
            size: n,
            heap: MinMaxHeap::with_capacity(n),
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        if self.heap.len() < self.size {
            self.heap.push(value);
            true
        } else if value > *self.heap.peek_min().unwrap() {
            self.heap.replace_min(value);
            true
        } else {
            false
        }
    }
}

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
    pub fn drain_top_items(mut self, n: usize) -> Vec<KeyValue<K, V>> {
        let mut heap = SizedHeap::new(n);
        for (fragment, node) in self.children.drain() {
            node.add_to_heap(vec![fragment], &mut heap);
        }
        heap.heap.into_vec_desc()
    }

    fn add_to_heap(mut self, key: Vec<K>, heap: &mut SizedHeap<KeyValue<K, V>>) {
        let item = KeyValue {
            key: key.to_owned(),
            value: self.value,
        };
        if heap.insert(item) {
            for (fragment, node) in self.children.drain() {
                let mut key = key.to_owned();
                key.push(fragment);
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
    assert_eq!(*trie.get(vec!["ls", "-l", "/home"]).unwrap(), 2);
    assert_eq!(*trie.get(vec!["ls", "-l", "/dev"]).unwrap(), 1);
    assert_eq!(*trie.get(vec!["cd", "/dev"]).unwrap(), 1);

    // Intermediate values
    assert_eq!(*trie.get(vec!["ls", "-l"]).unwrap(), 3);
    assert_eq!(*trie.get(vec!["ls"]).unwrap(), 4);
}
