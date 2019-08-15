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
