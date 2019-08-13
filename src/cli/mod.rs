use structopt::StructOpt;

pub use suggest::suggest;

use crate::cli::suggest::SuggestArgs;

mod suggest;

/// Represents available sub-commands
#[derive(Debug, StructOpt)]
pub enum Cli {
    #[structopt(name = "suggest")]
    Suggest(SuggestArgs),
}
