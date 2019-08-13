use std::{io::{Seek, SeekFrom, Write}, iter::FromIterator};

use tempfile::tempfile;

use super::{ExecutedCommand, History};

#[test]
fn history_dated_format() {
    let input = vec![
        ": 1556993411:0;cargo fmt",
        ": 1556991281:0;cargo build --release",
    ];
    let expected = vec![
        ExecutedCommand {
            args: vec!["cargo".to_string(), "fmt".to_string()],
            time: Some(1556993411),
        },
        ExecutedCommand {
            args: vec![
                "cargo".to_string(),
                "build".to_string(),
                "--release".to_string(),
            ],
            time: Some(1556991281),
        },
    ];

    let hist = History::from_iter(input);
    assert_eq!(hist.commands, expected);
}

#[test]
fn history_undated_format() {
    let input = vec!["cargo fmt", "cargo build --release"];
    let expected = vec![
        ExecutedCommand {
            args: vec!["cargo".to_string(), "fmt".to_string()],
            time: None,
        },
        ExecutedCommand {
            args: vec![
                "cargo".to_string(),
                "build".to_string(),
                "--release".to_string(),
            ],
            time: None,
        },
    ];

    let hist = History::from_iter(input);
    assert_eq!(hist.commands, expected);
}

#[test]
fn history_from_file() {
    let mut f = tempfile().unwrap();
    write!(f, "cargo test\ncargo run\n").unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();

    let expected = vec![
        ExecutedCommand {
            args: vec!["cargo".to_string(), "test".to_string()],
            time: None,
        },
        ExecutedCommand {
            args: vec!["cargo".to_string(), "run".to_string()],
            time: None,
        },
    ];

    let hist = History::from(f);
    assert_eq!(hist.commands, expected);
}
