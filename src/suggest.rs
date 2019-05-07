use crate::command::{Command, Uses};
use crate::feature::FEATURES;
use crate::histfile::{self, Parsed};
use crate::options::SuggestArgs;
use crate::rank::{self, RankedCommand};
use crate::trie::Trie;
use prettytable::Table;
use std::cmp::{self, Ordering};
use std::error::Error;

pub fn suggest(args: SuggestArgs) -> Result<(), Box<Error>> {
    let parsed_commands = histfile::read_history(args.history_file)?;
    let num_parsed = parsed_commands.len();

    let mut trie: Trie<String, Uses> = Trie::new();
    for parsed in parsed_commands {
        let Parsed { args, time } = parsed;
        trie.update_path(args, |uses| uses.update(time));
    }

    let to_filter = cmp::max(args.count * 2, num_parsed / 10);
    let filtered: Vec<Command> = trie
        .drain_top_items(to_filter)
        .into_iter()
        .map(Command::from)
        .collect();
    let mut ranked = rank::rank(filtered, &FEATURES);
    ranked.sort_unstable_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap_or(Ordering::Equal));

    build_table(&ranked[..args.count]).printstd();
    Ok(())
}

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
