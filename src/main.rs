#[macro_use]
extern crate prettytable;

use structopt::StructOpt;

use options::Cli;

mod command;
mod feature;
mod histfile;
mod capped_heap;
mod options;
mod rank;
mod suggest;
mod trie;

fn main() {
    let result = match Cli::from_args() {
        Cli::Suggest(args) => suggest::suggest(args).map(|table| table.printstd()),
    };
    if let Err(e) = result {
        eprintln!("Encountered error: {}", e);
    }
}
