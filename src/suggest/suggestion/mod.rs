use crate::trie::KeyValue;

use super::executions::Executions;

#[cfg(test)]
mod tests;

/// Represents a suggested command to alias
#[derive(Debug, Eq, PartialEq)]
pub struct Suggestion {
    pub executions: Executions,
    pub command: String,
}

impl From<KeyValue<String, Executions>> for Suggestion {
    fn from(pair: KeyValue<String, Executions>) -> Self {
        Suggestion {
            executions: pair.value,
            command: pair.key.join(" "),
        }
    }
}