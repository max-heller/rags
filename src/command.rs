use crate::trie::KeyValue;
use chrono::{Duration, NaiveDateTime};
use core::cmp::Ordering;
use std::time::Duration as StdDuration;

#[derive(Debug)]
pub struct Command {
    pub args: Vec<String>,
    pub uses: Uses,
}

impl From<KeyValue<String, Uses>> for Command {
    fn from(pair: KeyValue<String, Uses>) -> Self {
        Command {
            args: pair.key,
            uses: pair.value,
        }
    }
}

#[derive(Debug, Default)]
pub struct Uses {
    pub count: u32,
    pub times: Option<Times>,
}

impl Uses {
    pub fn update(&self, time: Option<u32>) -> Self {
        Uses {
            count: self.count + 1,
            times: match time {
                Some(time) => Some(self.times.unwrap_or_default().update(time)),
                None => self.times,
            },
        }
    }
}

impl Ord for Uses {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Uses {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Uses {
    fn eq(&self, other: &Self) -> bool {
        self.count.eq(&other.count)
    }
}

impl Eq for Uses {}

#[derive(Debug, Default, Clone, Copy)]
pub struct Times {
    count: u32,
    mean: f64,
    m2: f64,
}

impl Times {
    pub fn update(&self, time: u32) -> Self {
        let time = time as f64;
        let count = self.count + 1;
        let delta = time - self.mean;
        let mean = self.mean + delta / (count as f64);
        Times {
            count: count,
            mean: mean,
            m2: self.m2 + delta * (time - mean),
        }
    }

    /// Returns the mean call time as a unix timestamp
    pub fn mean_raw(&self) -> f64 {
        self.mean
    }

    /// Returns the mean call time as a parsed NaiveDateTime
    pub fn mean(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.mean as i64, 0)
    }

    /// Returns the variance of call times in seconds
    pub fn variance_raw(&self) -> f64 {
        self.m2 / (self.count as f64)
    }

    /// Returns the standard deviation of call times as a Duration
    pub fn std(&self) -> Duration {
        let duration = StdDuration::from_secs(self.variance_raw().sqrt() as u64);
        Duration::from_std(duration).unwrap()
    }
}

#[test]
fn call_times_mean_std() {
    let inputs = vec![
        vec![5.],
        vec![2., 4.],
        vec![2., 4., 6.],
        vec![123., 523411., 2343333., 44444., 23417., 234.],
    ];
    for times in inputs {
        let sum: f64 = times.iter().sum();
        let mean = sum / times.len() as f64;
        let sum_sqs: f64 = times.iter().map(|x| f64::powi(x - mean, 2)).sum();
        let variance = sum_sqs / times.len() as f64;
        let actual = times
            .into_iter()
            .fold(Times::default(), |times, time| times.update(time as u32));
        assert_eq!(mean, actual.mean);
        assert_eq!(variance.round(), actual.variance_raw().round());
    }
}
