use std::path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct SuggestArgs {
    pub count: u32,

    #[structopt(name = "Shell History Path", short = "p")]
    pub history_file: Option<path::PathBuf>,
}

#[derive(Debug, StructOpt)]
pub enum Cli {
    #[structopt(name = "suggest")]
    Suggest(SuggestArgs),
}
