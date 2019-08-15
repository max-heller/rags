use super::{Executions, Suggestion};

#[test]
fn test_new() {
    assert_eq!(
        Suggestion::new(
            vec!["cargo".to_string(), "run".to_string()],
            Executions::default(),
        ),
        Suggestion {
            command: "cargo run".to_string(),
            length: 9,
            args: vec!["cargo".to_string(), "run".to_string()],
            executions: Executions::default(),
        }
    );
}

#[test]
fn order() {
    let a1 = vec!["cargo".to_string()];
    let a2 = vec!["cargo".to_string(), "run".to_string()];
    let a3 = vec!["abc".to_string()];
    let a4 = vec!["abcdefghijklmnop".to_string()];
    let a5 = vec!["cargo".to_string(), "run".to_string(), "--release".to_string()];
    let a6 = vec!["cargo".to_string(), "run".to_string(), "--debug".to_string()];

    let exec_high = Executions {
        count: 100,
        last_executed: None,
    };
    let exec_med = Executions {
        count: 50,
        last_executed: None,
    };
    let exec_low = Executions {
        count: 10,
        last_executed: None,
    };

    let s = Suggestion::new;

    // Sorting based on length and number of args
    assert!(s(a3.to_owned(), exec_high) < s(a2.to_owned(), exec_high));
    assert!(s(a3.to_owned(), exec_high) < s(a4.to_owned(), exec_high));
    assert!(s(a3.to_owned(), exec_high) < s(a1.to_owned(), exec_high));
    assert!(s(a2.to_owned(), exec_high) < s(a4.to_owned(), exec_high));
    assert!(s(a2.to_owned(), exec_high) < s(a5.to_owned(), exec_high));
    assert!(s(a4.to_owned(), exec_high) < s(a5.to_owned(), exec_high));
    assert!(s(a5.to_owned(), exec_high) > s(a6.to_owned(), exec_high));

    // Sorting based on executions
    assert!(s(a5.to_owned(), exec_high) > s(a5.to_owned(), exec_med));
    assert!(s(a5.to_owned(), exec_high) > s(a5.to_owned(), exec_low));
    assert!(s(a5.to_owned(), exec_med) > s(a5.to_owned(), exec_low));
}
