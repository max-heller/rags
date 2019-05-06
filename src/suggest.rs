use crate::command::{Command, CommandRank};
use crate::histfile;
use crate::options::SuggestArgs;
use crate::trie::{KeyValue, Trie};
use prettytable::{Row, Table};
use std::io;

pub fn suggest(args: SuggestArgs) -> io::Result<()> {
    let commands = histfile::read_history(args.history_file)?;

    let mut trie: Trie<String, CommandRank> = Trie::new();
    for cmd in commands {
        let Command { args, time } = cmd;
        trie.update_path(args, |rank| rank.update(time));
    }

    let results = trie.get_top_values(args.count);
    build_table(results).printstd();
    Ok(())
}

fn build_table<'a>(items: Vec<KeyValue<'a, String, CommandRank>>) -> Table {
    let mut table = table!(["Command", "Uses", "Average Time of Use", "Time \u{03C3}"]);
    for item in items {
        let cmd: Vec<String> = item.key.into_iter().map(|s| s.to_string()).collect();
        let cmd: String = cmd.join(" ");
        let rank: &CommandRank = item.value;
        table.add_row(format_row(cmd, rank));
    }
    table
}

fn format_row(command: String, rank: &CommandRank) -> Row {
    match rank.times {
        Some(times) => row![
            command,
            rank.count,
            times.mean(),
            format!("{} hours", times.std().num_hours())
        ],
        None => row![command, rank.count, "N/A", "N/A"],
    }
}
