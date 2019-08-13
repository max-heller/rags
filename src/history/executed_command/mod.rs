use regex::Regex;

#[cfg(test)]
mod tests;

/// Regex pattern for parsing lines of history files
pub const HIST_PATTERN: &str = r"^(: (?P<time>\d{10}):\d+;)?(?P<cmd>.*)";

/// A parsed command from a line of a history file
#[derive(Debug, PartialEq)]
pub struct ExecutedCommand {
    /// Arguments of the command
    pub args: Vec<String>,
    /// Time of execution
    pub time: Option<u32>,
}

impl ExecutedCommand {
    /// Attempts to parse a command from a line of text
    pub fn try_parse(line: &str, re: &Regex) -> Option<Self> {
        re.captures(line).and_then(|caps| {
            caps.name("cmd").and_then(|cmd| {
                Some(ExecutedCommand {
                    args: cmd
                        .as_str()
                        .split_whitespace()
                        .map(str::to_string)
                        .collect(),
                    time: caps.name("time").map(|time| time.as_str().parse().unwrap()),
                })
            })
        })
    }
}