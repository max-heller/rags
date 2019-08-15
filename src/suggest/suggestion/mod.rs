use std::cmp::Ordering;

use super::executions::Executions;

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod tests;

/// Represents a suggested command to alias
#[derive(Debug, Eq, PartialEq)]
pub struct Suggestion {
    pub command: String,
    pub length: usize,
    pub args: Vec<String>,
    pub executions: Executions,
}

impl Suggestion {
    const BASELINE_LEN: usize = 10;
    const BASELINE_ARGS: usize = 2;

    /// Initializes a `Suggestion`
    pub fn new<T>(args: Vec<T>, executions: Executions) -> Self
        where T: Into<String> {
        let args: Vec<String> = args.into_iter().map(|arg| arg.into()).collect();
        let command = args.join(" ");
        let length = command.len();
        Suggestion {
            command,
            length,
            args,
            executions,
        }
    }
}

impl Ord for Suggestion {
    fn cmp(&self, other: &Self) -> Ordering {
        let length_ordering = self.length.cmp(&other.length);
        let argc_ordering = self.args.len().cmp(&other.args.len());
        let exec_ordering = self.executions.cmp(&other.executions);

        let exec_count_avg = (other.executions.count as f64 + self.executions.count as f64) / 2.0;
        let exec_count_diff =
            (other.executions.count as f64 - self.executions.count as f64).abs() / exec_count_avg;

        if self.length < Self::BASELINE_LEN || other.length < Self::BASELINE_LEN {
            // Very short command--prioritize longer
            length_ordering.then(argc_ordering).then(exec_ordering)
        } else if self.args.len() < Self::BASELINE_ARGS || other.args.len() < Self::BASELINE_ARGS {
            // Few arguments--prioritize more
            argc_ordering.then(length_ordering).then(exec_ordering)
        } else if exec_count_diff < 0.2 {
            // Similar executions--prioritize length and argc
            argc_ordering.then(length_ordering).then(exec_ordering)
        } else {
            // Prioritize executions
            exec_ordering.then(argc_ordering).then(length_ordering)
        }
    }
}

impl PartialOrd for Suggestion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
