use structopt::StructOpt;

pub use suggest::suggest;

use crate::cli::suggest::SuggestArgs;

mod suggest;

/// Rust Alias Generator for Shells
#[derive(Debug, StructOpt)]
pub enum Cli {
    /// Generates a table of suggested commands to alias
    #[structopt(name = "suggest")]
    Suggest(SuggestArgs),
}
