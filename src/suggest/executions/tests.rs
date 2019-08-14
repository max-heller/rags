use chrono::{Local, TimeZone};

use super::Executions;

#[test]
fn default() {
    assert_eq!(
        Executions::default(),
        Executions {
            count: 0,
            last_executed: None,
        }
    );
}

#[test]
fn updating_no_last_executed() {
    let base = Executions::default();
    assert_eq!(
        base.update(Some(5)),
        Executions {
            count: 1,
            last_executed: Some(5),
        }
    );
    assert_eq!(
        base.update(None),
        Executions {
            count: 1,
            last_executed: None,
        }
    );
}

#[test]
fn updating_existing_last_executed() {
    let base = Executions {
        count: 1,
        last_executed: Some(5),
    };
    assert_eq!(
        base.update(None),
        Executions {
            count: 2,
            last_executed: Some(5),
        }
    );
    assert_eq!(
        base.update(Some(3)),
        Executions {
            count: 2,
            last_executed: Some(5),
        }
    );
    assert_eq!(
        base.update(Some(6)),
        Executions {
            count: 2,
            last_executed: Some(6),
        }
    );
}

#[test]
fn representation() {
    let executions = Executions {
        count: 1,
        last_executed: Some(1565737322),
    };
    assert_eq!(executions.last_executed().unwrap(), Local.timestamp(1565737322, 0));
    assert!(executions.last_executed_str().unwrap().starts_with("2019-08"));
}