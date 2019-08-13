use std::cmp::{self, Ordering};

use failure::Error;
use prettytable::Table;

use crate::command::{Command, Uses};
use crate::feature::FEATURES;
use crate::histfile::{History, ParsedCommand};
use crate::options::SuggestArgs;
use crate::rank::{self, RankedCommand};
use crate::trie::Trie;

/// Produces a table of suggested aliases
pub fn suggest(args: SuggestArgs) -> Result<Table, Error> {
    // Parse commands from history file
    let history = History::from_file(args.history_file)?;
    let num_commands = history.commands.len();

    // Insert commands into trie, counting the frequency with which prefixes of commands are used
    // e.g. `cargo run` counts as a usage of `cargo run` and `cargo`
    let mut trie: Trie<String, Uses> = Trie::new();
    for parsed in history.commands {
        let ParsedCommand { args, time } = parsed;
        trie.update_path(args, |uses| uses.update(time));
    }

    let to_filter = cmp::max(args.count * 2, num_commands / 10);
    let filtered: Vec<Command> = trie
        .drain_top_items(to_filter)
        .into_iter()
        .map(Command::from)
        .collect();
    let mut ranked = rank::rank(filtered, &FEATURES);
    ranked.sort_unstable_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap_or(Ordering::Equal));

    Ok(build_table(&ranked[..args.count]))
}

/// Converts a list of ranked commands into a table
fn build_table(results: &[RankedCommand]) -> Table {
    let mut table = table!(["Command", "Uses", "Rank"]);
    for result in results {
        table.add_row(row![
            result.command.args.join(" "),
            result.command.uses.count,
            result.rank
        ]);
    }
    table
}
