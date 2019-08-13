use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use failure::{Error, ResultExt};
use regex::Regex;

/// Regex pattern for parsing lines of history files
const HIST_PATTERN: &str = r"^(: (?P<time>\d{10}):\d+;)?(?P<cmd>.*)";

/// Represents a history file
#[derive(Debug, PartialEq)]
pub struct History {
    pub commands: Vec<ParsedCommand>,
}

impl<Iter, Item> From<Iter> for History
where
    Iter: IntoIterator<Item=Item>,
    Item: AsRef<str>,
{
    /// Parses commands from an iterator of lines
    ///
    /// Lines that cannot be parsed are discarded
    fn from(lines: Iter) -> Self {
        let re = Regex::new(HIST_PATTERN).unwrap();
        let commands = lines
            .into_iter()
            .filter_map(|line| ParsedCommand::try_parse(line.as_ref(), &re))
            .collect();
        History { commands }
    }
}

impl History {
    /// Attempts to read and parse lines from a history file
    pub fn from_file(path: PathBuf) -> Result<Self, Error> {
        let f = File::open(path).context("Unable to open history file")?;
        let lines = BufReader::new(f).lines().filter_map(Result::ok);
        Ok(Self::from(lines))
    }
}

/// A parsed command from a line of a history file
#[derive(Debug, PartialEq)]
pub struct ParsedCommand {
    /// The arguments of the command
    pub args: Vec<String>,
    /// The time when the command was executed
    pub time: Option<u32>,
}

impl ParsedCommand {
    /// Attempts to parse a command from a line of text
    fn try_parse(line: &str, re: &Regex) -> Option<Self> {
        re.captures(line).and_then(|caps| {
            caps.name("cmd").and_then(|cmd| {
                Some(ParsedCommand {
                    args: cmd
                        .as_str()
                        .split_whitespace()
                        .map(str::to_string)
                        .collect(),
                    time: caps.name("time").map(|time| time.as_str().parse().unwrap()),
                })
            })
        })
    }
}

#[test]
fn parse_dated_format() {
    let input = vec![
        ": 1556993411:0;cargo fmt",
        ": 1556991281:0;cargo build --release",
    ];
    let expected = vec![
        ParsedCommand {
            args: vec!["cargo".to_string(), "fmt".to_string()],
            time: Some(1556993411 as u32),
        },
        ParsedCommand {
            args: vec![
                "cargo".to_string(),
                "build".to_string(),
                "--release".to_string(),
            ],
            time: Some(1556991281 as u32),
        },
    ];

    let hist = History::from(input);
    assert_eq!(hist.commands, expected);
    assert!(hist.commands.iter().all(|cmd| cmd.time.is_some()));
}

#[test]
fn parse_undated_format() {
    let input = vec!["cargo fmt", "cargo build --release"];
    let expected = vec![
        ParsedCommand {
            args: vec!["cargo".to_string(), "fmt".to_string()],
            time: None,
        },
        ParsedCommand {
            args: vec![
                "cargo".to_string(),
                "build".to_string(),
                "--release".to_string(),
            ],
            time: None,
        },
    ];

    let hist = History::from(input);
    assert_eq!(hist.commands, expected);
    assert!(hist.commands.iter().all(|cmd| cmd.time.is_none()));
}
