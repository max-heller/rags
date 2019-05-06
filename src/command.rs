use chrono::{Duration, NaiveDateTime};
use core::cmp::Ordering;
use std::time::Duration as StdDuration;

#[derive(Debug, PartialEq)]
pub struct Command {
    pub args: Vec<String>,
    pub time: Option<u32>,
}

#[derive(Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
pub struct CommandRank {
    pub count: u32,
    pub times: Option<CallTimes>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CallTimes {
    count: u32,
    mean: f64,
    m2: f64,
}

impl CommandRank {
    pub fn update(&self, time: Option<u32>) -> Self {
        CommandRank {
            count: self.count + 1,
            times: match time {
                Some(time) => Some(self.times.unwrap_or_default().update(time)),
                None => self.times,
            },
        }
    }
}

impl CallTimes {
    pub fn update(&self, time: u32) -> Self {
        let time = time as f64;
        let count = self.count + 1;
        let delta = time - self.mean;
        let mean = self.mean + delta / (count as f64);
        CallTimes {
            count: count,
            mean: mean,
            m2: self.m2 + delta * (time - mean),
        }
    }

    pub fn mean(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.mean as i64, 0)
    }

    /// Returns the variance of call times in seconds
    pub fn variance(&self) -> f64 {
        self.m2 / (self.count as f64)
    }

    /// Returns the standard deviation of call times as a Duration
    pub fn std(&self) -> Duration {
        let duration = StdDuration::from_secs(self.variance().sqrt() as u64);
        Duration::from_std(duration).unwrap()
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

#[test]
fn call_times_mean_std() {
    let inputs = vec![
        vec![5.0],
        vec![2.0, 4.0],
        vec![2.0, 4.0, 6.0],
        vec![123.0, 523411.0, 2343333.0, 44444.0, 23417.0, 234.0],
    ];
    for times in inputs {
        let sum: f64 = times.iter().sum();
        let mean = sum / times.len() as f64;
        let sum_sqs: f64 = times.iter().map(|x| f64::powi(x - mean, 2)).sum();
        let variance = sum_sqs / times.len() as f64;
        let actual = times.into_iter().fold(CallTimes::default(), |times, time| {
            times.update(time as u32)
        });
        assert_eq!(mean, actual.mean);
        assert_eq!(variance.round(), actual.variance().round());
    }
}
