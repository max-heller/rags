use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::FromIterator,
};

use regex::Regex;

pub use executed_command::ExecutedCommand;

mod executed_command;
#[cfg(test)]
mod tests;

/// Represents a history file
#[derive(Debug, PartialEq)]
pub struct History {
    pub commands: Vec<ExecutedCommand>,
}

impl<T: AsRef<str>> FromIterator<T> for History {
    /// Parses commands from an iterator, discarding any lines that can't be parsed
    fn from_iter<I: IntoIterator<Item=T>>(lines: I) -> Self {
        let re = Regex::new(ExecutedCommand::PATTERN).unwrap();
        let commands = lines
            .into_iter()
            .filter_map(|line| ExecutedCommand::try_parse(line.as_ref(), &re))
            .collect();
        History { commands }
    }
}

impl From<File> for History {
    /// Attempts to read and parse lines from a history file
    fn from(file: File) -> Self {
        BufReader::new(file).lines().filter_map(Result::ok).collect()
    }
}
