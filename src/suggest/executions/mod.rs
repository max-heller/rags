use std::cmp::{max, Ordering};

use chrono::{DateTime, offset::Local, TimeZone};

#[cfg(test)]
mod tests;

/// Represents the executions of a command
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Executions {
    /// How many times the command has been executed
    pub count: u32,
    /// The time when the command was last executed
    pub last_executed: Option<u32>,
}

impl Executions {
    /// Produces updated struct incorporating an additional execution
    pub fn update(&self, time: Option<u32>) -> Self {
        Executions {
            count: self.count + 1,
            last_executed: self
                .last_executed
                .map(|old| time.map_or(old, |new| max(old, new)))
                .or(time),
        }
    }

    /// Produces the `NaiveDateTime` at which the command was last executed
    fn last_executed(&self) -> Option<DateTime<Local>> {
        self.last_executed
            .map(|time| Local.timestamp(time as i64, 0))
    }

    /// Produces a human readable time at which the command was last executed
    pub fn last_executed_str(&self) -> Option<String> {
        self.last_executed()
            .map(|datetime| datetime.format("%Y-%m-%d %I:%M%p").to_string())
    }
}

impl Ord for Executions {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Executions {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
