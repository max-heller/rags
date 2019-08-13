use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Cli {
    #[structopt(name = "suggest")]
    Suggest(SuggestArgs),
}

#[derive(Debug, StructOpt)]
pub struct SuggestArgs {
    #[structopt(name = "History file")]
    pub history_file: PathBuf,
    #[structopt(name = "Number of aliases to suggest")]
    pub count: usize,
}
