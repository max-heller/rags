use options::Cli;
use std::error::Error;
use structopt::StructOpt;
mod command;
mod feature;
mod histfile;
mod options;
mod rank;
mod suggest;
mod trie;
#[macro_use]
extern crate prettytable;

fn main() -> Result<(), Box<Error>> {
    match Cli::from_args() {
        Cli::Suggest(args) => suggest::suggest(args),
    }
}
