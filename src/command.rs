use std::cmp::Ordering;

#[derive(Debug, Hash)]
pub struct Command {
    pub s: String,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        Command { s: s.to_string() }
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Command) -> bool {
        self.s.len() == other.s.len()
    }
}

impl Eq for Command {}

impl PartialOrd for Command {
    fn partial_cmp(&self, other: &Command) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Command {
    fn cmp(&self, other: &Command) -> Ordering {
        self.s.len().cmp(&other.s.len())
    }
}
