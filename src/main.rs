use options::Cli;
use std::io;
use structopt::StructOpt;
mod command;
mod counting_trie;
mod histfile;
mod options;
mod suggest;

fn main() -> io::Result<()> {
    match Cli::from_args() {
        Cli::Suggest(args) => suggest::suggest(args),
    }
}
