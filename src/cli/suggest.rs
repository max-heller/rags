use std::fs::File;
use std::path::PathBuf;

use failure::{Error, ResultExt};
use prettytable::Table;
use structopt::StructOpt;

use crate::history::History;
use crate::suggest::build_table;

/// Represents command line arguments for the `suggest` sub-command
#[derive(Debug, StructOpt)]
pub struct SuggestArgs {
    /// Path to history file
    #[structopt(name = "histfile")]
    pub history_file: PathBuf,
    /// Number of aliases to suggest
    #[structopt(name = "n", default_value = "5")]
    pub count: usize,
}

/// Outputs a table of suggested command aliases
pub fn suggest(args: SuggestArgs) -> Result<Table, Error> {
    let hist_file = File::open(args.history_file).context("Unable to open history file")?;
    let history = History::from(hist_file);
    let suggestions = crate::suggest::suggest(history, args.count);
    Ok(build_table(suggestions))
}