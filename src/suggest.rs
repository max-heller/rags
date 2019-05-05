use crate::histfile;
use crate::options::SuggestArgs;
use crate::trie::Trie;
use chrono::{Duration, NaiveDateTime};
use core::cmp::Ordering;
use histfile::Command;
use std::io;
use std::time::Duration as StdDuration;

pub fn suggest(args: SuggestArgs) -> io::Result<()> {
    let commands = histfile::read_history(args.history_file)?;

    let mut trie: Trie<String, CommandRank> = Trie::new();
    for cmd in commands {
        match cmd {
            Command { args, time } => {
                trie.update_path(args, |rank: &CommandRank| rank.update(time));
            }
        }
    }

    let mut table = table!(["Command", "Uses", "Average Time of Use", "Time STD"]);
    for item in trie.get_top_values(args.count) {
        let cmd: Vec<String> = item.key.into_iter().map(|s| s.to_string()).collect();
        let rank: &CommandRank = item.value;
        table.add_row(row![cmd.join(" "), rank.count, rank.times.mean(), rank.times.std().num_days()]);
    }
    table.printstd();
    Ok(())
}

#[derive(Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
struct CommandRank {
    count: u32,
    times: CallTimes,
}

#[derive(Debug, Default)]
struct CallTimes {
    count: u32,
    mean: f64,
    m2: f64,
}

impl CommandRank {
    fn update(&self, time: Option<u32>) -> Self {
        CommandRank {
            count: self.count + 1,
            times: self.times.update(time),
        }
    }
}

impl CallTimes {
    fn update(&self, time: Option<u32>) -> Self {
        match time {
            Some(time) => {
                let count = self.count + 1;
                let delta = time as f64 - self.mean;
                let mean = self.mean + delta / count as f64;
                let delta2 = time as f64 - mean;
                CallTimes {
                    count: count,
                    mean: mean,
                    m2: delta * delta2,
                }
            }
            None => CallTimes {
                count: self.count,
                mean: self.mean,
                m2: self.m2,
            },
        }
    }

    fn mean(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.mean as i64, 0)
    }

    fn variance(&self) -> f64 {
        self.m2 / self.count as f64
    }

    fn std(&self) -> Duration {
        Duration::from_std(StdDuration::from_secs(self.variance().sqrt() as u64)).unwrap()
    }
}

impl Ord for CallTimes {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.mean.partial_cmp(&other.mean) {
            Some(mean_order) => mean_order,
            None => match self.variance().partial_cmp(&other.variance()) {
                Some(var_order) => var_order,
                None => self.count.cmp(&other.count),
            },
        }
    }
}

impl PartialOrd for CallTimes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CallTimes {
    fn eq(&self, other: &Self) -> bool {
        self.mean == other.mean && self.variance() == other.variance() && self.count == other.count
    }
}

impl Eq for CallTimes {}
