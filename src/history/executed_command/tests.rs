use regex::Regex;

use super::{ExecutedCommand, HIST_PATTERN};

#[test]
fn parse_dated_format() {
    let re = Regex::new(HIST_PATTERN).unwrap();
    assert_eq!(
        ExecutedCommand::try_parse(": 1556993411:0;cargo fmt", &re),
        Some(ExecutedCommand {
            args: vec!["cargo".to_string(), "fmt".to_string()],
            time: Some(1556993411),
        })
    );
    assert_eq!(
        ExecutedCommand::try_parse(": 1556991281:0;cargo build --release", &re),
        Some(ExecutedCommand {
            args: vec![
                "cargo".to_string(),
                "build".to_string(),
                "--release".to_string(),
            ],
            time: Some(1556991281),
        })
    );
}

#[test]
fn parse_undated_format() {
    let re = Regex::new(HIST_PATTERN).unwrap();
    assert_eq!(
        ExecutedCommand::try_parse("cargo fmt", &re),
        Some(ExecutedCommand {
            args: vec!["cargo".to_string(), "fmt".to_string()],
            time: None,
        })
    );
    assert_eq!(
        ExecutedCommand::try_parse("cargo build --release", &re),
        Some(ExecutedCommand {
            args: vec![
                "cargo".to_string(),
                "build".to_string(),
                "--release".to_string(),
            ],
            time: None,
        })
    );
}