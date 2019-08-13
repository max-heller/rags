use super::{Executions, KeyValue, Suggestion};

#[test]
fn test_from() {
    let kv = KeyValue {
        key: vec!["abc".to_string(), "123".to_string()],
        value: Executions::default().update(Some(5)),
    };
    let expected = Suggestion {
        executions: Executions::default().update(Some(5)),
        command: "abc 123".to_string(),
    };
    assert_eq!(
        Suggestion::from(kv),
        expected
    );
}