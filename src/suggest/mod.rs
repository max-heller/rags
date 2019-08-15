use prettytable::Table;

use executions::Executions;
use suggestion::Suggestion;

use crate::capped_heap::CappedHeap;
use crate::history::{ExecutedCommand, History};
use crate::trie::Trie;

mod suggestion;
mod executions;
#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod tests;

impl Trie<String, Executions> {
    /// Produces a list of at most `n` `Suggestion`s sorted in descending order by value
    fn drain_top_items(mut self, n: usize) -> Vec<Suggestion> {
        let mut heap = CappedHeap::new(n);
        for (arg, node) in self.children.drain() {
            node.add_to_heap(vec![arg], &mut heap);
        }
        heap.heap.into_vec_desc()
    }

    /// Adds the current node and its children to a `CappedHeap`
    fn add_to_heap(mut self, args: Vec<String>, heap: &mut CappedHeap<Suggestion>) {
        // Add current node to the heap
        let suggestion = Suggestion::new(args.to_owned(), self.value);
        heap.insert(suggestion);

        // Add children to the heap
        for (arg, node) in self.children.drain() {
            let mut args = args.to_owned();
            args.push(arg);
            node.add_to_heap(args, heap);
        }
    }
}

/// Produces an iterator of `n` suggested commands to alias
pub fn suggest(history: History, n: usize) -> impl Iterator<Item=Suggestion> {
    // Insert commands into trie, counting the frequency with which prefixes of commands are used
    // e.g. `cargo run` counts as a usage of `cargo run` and `cargo`
    let mut trie: Trie<String, Executions> = Trie::new();
    for parsed in history.commands {
        let ExecutedCommand { args, time } = parsed;
        trie.update_path(args, |uses| uses.update(time));
    }

    // Generate suggestions
    trie.drain_top_items(n).into_iter().map(Suggestion::from)
}

/// Converts an iterator of suggestions into a table
pub fn build_table<I>(suggestions: I) -> Table
    where
        I: IntoIterator<Item=Suggestion>,
{
    let mut table = table!(["Uses", "Last Used", "Command"]);
    for suggestion in suggestions {
        table.add_row(row![
            suggestion.executions.count,
            suggestion
                .executions
                .last_executed_str()
                .unwrap_or("Unknown".to_string()),
            suggestion.command,
        ]);
    }
    table
}
