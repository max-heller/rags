use dirs;
use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
pub struct Parsed {
    pub args: Vec<String>,
    pub time: Option<u32>,
}

const HIST_PATTERN: &str = r"^(: (?P<time>\d{10}):\d+;)?(?P<cmd>.*)";

pub fn read_history(path: Option<std::path::PathBuf>) -> Result<Vec<Parsed>, Box<Error>> {
    let f = File::open(match path {
        Some(path) => path,
        None => {
            let mut path = dirs::home_dir().unwrap();
            path.push(".histfile");
            path
        }
    })?;

    let line_iter = BufReader::new(f).lines().filter_map(Result::ok);
    Ok(parse_commands(line_iter))
}

pub fn parse_commands<T>(lines: T) -> Vec<Parsed>
where
    T: IntoIterator<Item = String>,
{
    let re = Regex::new(HIST_PATTERN).unwrap();
    let parse_command = |line: String| -> Option<Parsed> {
        re.captures(&line).and_then(|caps| {
            caps.name("cmd").and_then(|cmd| {
                Some(Parsed {
                    args: cmd
                        .as_str()
                        .split_whitespace()
                        .map(str::to_string)
                        .collect(),
                    time: caps.name("time").map(|time| time.as_str().parse().unwrap()),
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
        Parsed {
            args: vec!["cargo".to_string(), "fmt".to_string()],
            time: Some(1556993411 as u32),
        },
        Parsed {
            args: vec![
                "cargo".to_string(),
                "build".to_string(),
                "--release".to_string(),
            ],
            time: Some(1556991281 as u32),
        },
    ];
    assert_eq!(parse_commands(input), expected);
    assert!(expected.iter().all(|cmd| cmd.time.is_some()));
}

#[test]
fn parse_undated_format() {
    let input = vec!["cargo fmt".to_string(), "cargo build --release".to_string()];
    let expected = vec![
        Parsed {
            args: vec!["cargo".to_string(), "fmt".to_string()],
            time: None,
        },
        Parsed {
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
