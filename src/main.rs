#[macro_use]
extern crate prettytable;

use structopt::StructOpt;

use cli::Cli;

mod history;
mod capped_heap;
#[cfg_attr(tarpaulin, skip)]
mod cli;
mod suggest;
mod trie;

#[cfg_attr(tarpaulin, skip)]
fn main() {
    let result = match Cli::from_args() {
        Cli::Suggest(args) => cli::suggest(args).map(|table| table.printstd()),
    };
    if let Err(e) = result {
        eprintln!("Encountered error: {}", e);
    }
}
