use prettytable::Table;

use executions::Executions;
use suggestion::Suggestion;

use crate::history::{ExecutedCommand, History};
use crate::trie::Trie;

mod suggestion;
mod executions;
#[cfg(test)]
mod tests;

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
