use crate::counting_trie::CountingTrie;
use crate::histfile;
use crate::options::SuggestArgs;
use std::io;

pub fn suggest(args: SuggestArgs) -> io::Result<()> {
    let commands = histfile::read_history(args.history_file)?;
    let command_args = commands.iter().map(|s| s.split_whitespace());
    let trie = CountingTrie::from(command_args);
    println!("{:?}", trie.get_count("make".split_whitespace()));
    Ok(())
}
