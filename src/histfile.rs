use chrono::NaiveDateTime;
use dirs;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

const HIST_PATTERN: &str = r"^(: (?P<time>\d{10}):\d+;)?(?P<cmd>.*)";

#[derive(Debug, PartialEq)]
pub struct Command {
    pub args: Vec<String>,
    pub time: Option<NaiveDateTime>,
}

impl From<String> for Command {
    fn from(string: String) -> Self {
        Command {
            args: string.split_whitespace().map(str::to_string).collect(),
            time: None,
        }
    }
}

impl Into<String> for Command {
    fn into(self) -> String {
        self.args.join(" ")
    }
}

impl IntoIterator for Command {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;
    fn into_iter(self) -> Self::IntoIter {
        self.args.into_iter()
    }
}

pub fn read_history(path: Option<std::path::PathBuf>) -> io::Result<Vec<Command>> {
    let f = match path {
        Some(path) => File::open(path),
        None => {
            let mut path = dirs::home_dir().unwrap();
            path.push(".histfile");
            File::open(path)
        }
    }?;

    let line_iter = BufReader::new(f).lines().filter_map(Result::ok);
    Ok(parse_commands(line_iter))
}

pub fn parse_commands<T>(lines: T) -> Vec<Command>
where
    T: IntoIterator<Item = String>,
{
    let re = Regex::new(HIST_PATTERN).unwrap();
    let parse_command = |line: String| -> Option<Command> {
        re.captures(&line).and_then(|caps| {
            caps.name("cmd").and_then(|cmd| {
                Some(Command {
                    args: cmd
                        .as_str()
                        .split_whitespace()
                        .map(str::to_string)
                        .collect(),
                    time: caps.name("time").and_then(|time| {
                        NaiveDateTime::from_timestamp_opt(time.as_str().parse().unwrap(), 0)
                    }),
                })
            })
        })
    };
    lines.into_iter().filter_map(parse_command).collect()
}

#[test]
fn parse_dated_format() {
    let input = vec![
        ": 1556993411:0;cargo fmt".to_string(),
        ": 1556991281:0;cargo build --release".to_string(),
    ];
    let expected = vec![
        Command {
            args: vec!["cargo".to_string(), "fmt".to_string()],
            time: NaiveDateTime::from_timestamp_opt(1556993411, 0),
        },
        Command {
            args: vec![
                "cargo".to_string(),
                "build".to_string(),
                "--release".to_string(),
            ],
            time: NaiveDateTime::from_timestamp_opt(1556991281, 0),
        },
    ];
    assert_eq!(parse_commands(input), expected);
    assert!(expected.iter().all(|cmd| cmd.time.is_some()));
}

#[test]
fn parse_undated_format() {
    let input = vec!["cargo fmt".to_string(), "cargo build --release".to_string()];
    let expected = vec![
        Command {
            args: vec!["cargo".to_string(), "fmt".to_string()],
            time: None,
        },
        Command {
            args: vec![
                "cargo".to_string(),
                "build".to_string(),
                "--release".to_string(),
            ],
            time: None,
        },
    ];
    assert_eq!(parse_commands(input), expected);
}
