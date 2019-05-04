use crate::counting_trie::CountingTrie;
use min_max_heap::MinMaxHeap;
use std::cmp::Ordering;

#[derive(Debug, Hash)]
pub struct Command {
    pub s: String,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        Command { s: s.to_string() }
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Command) -> bool {
        self.s.len() == other.s.len()
    }
}

impl Eq for Command {}

impl PartialOrd for Command {
    fn partial_cmp(&self, other: &Command) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Command {
    fn cmp(&self, other: &Command) -> Ordering {
        self.s.len().cmp(&other.s.len())
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CommandCount {
    pub count: u32,
    pub value: Command,
}

struct SizedMinMaxHeap {
    size: usize,
    heap: MinMaxHeap<CommandCount>,
}

impl SizedMinMaxHeap {
    pub fn new(size: usize) -> Self {
        SizedMinMaxHeap {
            size: size,
            heap: MinMaxHeap::with_capacity(size),
        }
    }

    pub fn insert(&mut self, val: CommandCount) {
        match self.heap.peek_min() {
            Some(min) => {
                if self.heap.len() < self.size {
                    self.heap.push(val);
                } else if val.count > min.count {
                    self.heap.replace_min(val);
                }
            }
            None => {
                self.heap.push(val);
            }
        }
    }
}

impl CountingTrie<&str> {
    pub fn get_n_top(self, n: usize) -> Vec<CommandCount> {
        let mut heap = SizedMinMaxHeap::new(n);
        for (start, subtrie) in self.children {
            subtrie.add_to_heap(&mut heap, start.to_string());
        }
        heap.heap.into_vec_desc()
    }

    fn add_to_heap(self, heap: &mut SizedMinMaxHeap, prefix: String) {
        for (fragment, subtrie) in self.children {
            let base: String = prefix.clone() + " " + fragment;
            heap.insert(CommandCount {
                count: subtrie.count,
                value: Command { s: base.clone() },
            });
            subtrie.add_to_heap(heap, base);
        }
    }
}
