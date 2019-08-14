use std::iter::FromIterator;

use crate::history::History;

use super::{build_table, executions::Executions, suggest, suggestion::Suggestion};

fn sample_hist() -> History {
    History::from_iter(&["abc 123", "cargo", "cargo run", "cargo run --release"])
}

fn sample_suggestions() -> Vec<Suggestion> {
    vec![
        Suggestion {
            executions: Executions {
                count: 3,
                last_executed: None,
            },
            command: "cargo".to_string(),
        },
        Suggestion {
            executions: Executions {
                count: 2,
                last_executed: None,
            },
            command: "cargo run".to_string(),
        },
    ]
}

#[test]
fn suggest_none() {
    assert_eq!(suggest(sample_hist(), 0).next(), None);
}

#[test]
fn suggest_one() {
    let suggestions: Vec<Suggestion> = suggest(sample_hist(), 1).collect();
    let expected = vec![Suggestion {
        executions: Executions {
            count: 3,
            last_executed: None,
        },
        command: "cargo".to_string(),
    }];
    assert_eq!(suggestions, expected);
}

#[test]
fn suggest_two() {
    let suggestions: Vec<Suggestion> = suggest(sample_hist(), 2).collect();
    assert_eq!(suggestions, sample_suggestions());
}

#[test]
fn suggest_all() {
    let mut suggestions: Vec<Suggestion> = suggest(sample_hist(), 5).collect();
    assert_eq!(suggestions.len(), 5);
    suggestions = suggest(sample_hist(), 20).collect();
    assert_eq!(suggestions.len(), 5);
}

#[test]
fn table_building() {
    let table = build_table(sample_suggestions());
    let expected = table!(
        ["Uses", "Last Used", "Command"],
        [3, "Unknown", "cargo"],
        [2, "Unknown", "cargo run"]
    );
    assert_eq!(table, expected);
}
