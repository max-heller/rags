use crate::command::Command;

pub struct Feature {
    pub weight: fn(f64) -> f64,
    pub evaluate: fn(&Command) -> f64,
}

pub const FEATURES: [Feature; 5] = [
    Feature {
        weight: |x| 10. * x,
        evaluate: |command: &Command| command.uses.count as f64,
    },
    Feature {
        weight: |x| 2. * x,
        evaluate: |command: &Command| command.args.join(" ").len() as f64,
    },
    Feature {
        weight: |x| 1. * x,
        evaluate: |command: &Command| command.args.len() as f64,
    },
    Feature {
        weight: |x| 1. * x,
        evaluate: |command: &Command| {
            command
                .uses
                .times
                .map(|times| times.mean_raw())
                .unwrap_or(0.)
        },
    },
    Feature {
        weight: |x| 1. * x,
        evaluate: |command: &Command| {
            command
                .uses
                .times
                .map(|times| times.variance_raw())
                .unwrap_or(0.)
        },
    },
];
