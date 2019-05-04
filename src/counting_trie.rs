use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct CountingTrie<K>
where
    K: Hash + Eq,
{
    pub count: u32,
    pub children: HashMap<K, CountingTrie<K>>,
}

impl<K> CountingTrie<K>
where
    K: Hash + Eq,
{
    pub fn new() -> CountingTrie<K> {
        CountingTrie {
            count: 0,
            children: HashMap::new(),
        }
    }

    pub fn insert<I>(&mut self, key: I)
    where
        I: IntoIterator<Item = K>,
    {
        let mut trie = key.into_iter().fold(self, |trie, fragment| {
            trie.count += 1;
            trie.children
                .entry(fragment)
                .or_insert_with(CountingTrie::new)
        });
        trie.count += 1;
    }

    pub fn get<I>(&self, key: I) -> Option<&CountingTrie<K>>
    where
        I: IntoIterator<Item = K>,
    {
        key.into_iter()
            .try_fold(self, |trie, fragment| trie.children.get(&fragment))
    }

    pub fn get_count<I>(&self, key: I) -> u32
    where
        I: IntoIterator<Item = K>,
    {
        match self.get(key) {
            Some(trie) => trie.count,
            None => 0,
        }
    }
}

impl<K, S, I> From<S> for CountingTrie<K>
where
    K: Hash + Eq,
    S: IntoIterator<Item = I>,
    I: IntoIterator<Item = K>,
{
    fn from(source: S) -> Self {
        let mut trie = CountingTrie::new();
        for item in source {
            trie.insert(item);
        }
        trie
    }
}

#[test]
fn trie_construction() {
    let mut trie = CountingTrie::new();
    let v1 = vec!["ls", "-l", "/home"];
    trie.insert(&v1);
    trie.insert(&v1);
    let v2 = vec!["ls", "-l", "/dev"];
    trie.insert(&v2);
    let v3 = vec!["cd", "/dev"];
    trie.insert(&v3);
    let v4 = vec!["ls"];
    trie.insert(&v4);

    eprintln!("{:#?}", trie);

    // Leaf values
    assert_eq!(trie.get_count(&v1), 2);
    assert_eq!(trie.get_count(&v2), 1);
    assert_eq!(trie.get_count(&v3), 1);
    assert_eq!(trie.get_count(&v1), 2);

    // Intermediate values
    assert_eq!(trie.get_count(&vec!["ls", "-l"]), 3);
    assert_eq!(trie.get_count(&vec!["ls"]), 4);

    // All values
    assert_eq!(trie.get_count(&vec![]), 5);
}
