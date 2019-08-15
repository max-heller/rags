use regex::Regex;

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod tests;

/// A parsed command from a line of a history file
#[derive(Debug, PartialEq)]
pub struct ExecutedCommand {
    /// Arguments of the command
    pub args: Vec<String>,
    /// Time of execution
    pub time: Option<u32>,
}

impl ExecutedCommand {
    /// Regex pattern for parsing lines of history files
    pub const PATTERN: &'static str = r"^(: (?P<time>\d{10}):\d+;)?(?P<cmd>.*)";

    /// Attempts to parse a command from a line of text
    pub fn try_parse(line: &str, re: &Regex) -> Option<Self> {
        re.captures(line).and_then(|caps| {
            caps.name("cmd").and_then(|cmd| {
                let split = cmd.as_str().split_whitespace();
                let args = split.map(str::to_string).collect();
                let time = caps.name("time").map(|time| time.as_str().parse().unwrap());
                Some(ExecutedCommand { args, time })
            })
        })
    }
}
