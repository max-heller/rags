use std::iter::FromIterator;

use crate::history::History;

use super::{build_table, executions::Executions, suggest, suggestion::Suggestion};

fn sample_hist() -> History {
    History::from_iter(&["abc 123", "cargo", "cargo run", "cargo run --release"])
}

fn sample_suggestions() -> Vec<Suggestion> {
    vec![
        Suggestion::new(
            vec!["cargo", "run", "--release"],
            Executions {
                count: 1,
                last_executed: None,
            },
        ),
        Suggestion::new(
            vec!["cargo", "run"],
            Executions {
                count: 2,
                last_executed: None,
            },
        ),
    ]
}

#[test]
fn suggest_none() {
    assert_eq!(suggest(sample_hist(), 0).next(), None);
}

#[test]
fn suggest_one() {
    let suggestions: Vec<Suggestion> = suggest(sample_hist(), 1).collect();
    let expected = &sample_suggestions()[..1];
    assert_eq!(&suggestions[..1], expected);
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
        [1, "Unknown", "cargo run --release"],
        [2, "Unknown", "cargo run"]
    );
    assert_eq!(table, expected);
}
