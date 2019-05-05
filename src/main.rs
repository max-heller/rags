use options::Cli;
use std::io;
use structopt::StructOpt;
mod command;
mod histfile;
mod options;
mod suggest;
mod trie;

fn main() -> io::Result<()> {
    match Cli::from_args() {
        Cli::Suggest(args) => suggest::suggest(args),
    }
}
