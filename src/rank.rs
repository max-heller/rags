use crate::command::Command;
use crate::feature::Feature;

#[derive(Debug)]
pub struct RankedCommand {
    pub command: Command,
    pub rank: f64,
}

pub fn rank(commands: Vec<Command>, features: &[Feature]) -> Vec<RankedCommand> {
    let evaluations: Vec<Vec<f64>> = commands
        .iter()
        .map(|command| {
            features
                .into_iter()
                .map(|feat| (feat.evaluate)(&command))
                .collect()
        })
        .collect();

    let mut iter = evaluations.iter();
    let first = match iter.next() {
        Some(first) => first,
        None => return vec![],
    };
    let float_max = |a, b| if a > b { a } else { b };
    let float_min = |a, b| if a < b { a } else { b };
    let (min, max) = iter.fold((first.to_vec(), first.to_vec()), |(min, max), eval| {
        let min = min
            .into_iter()
            .zip(eval.iter())
            .map(|(a, b)| float_min(a, *b))
            .collect();
        let max = max
            .into_iter()
            .zip(eval.iter())
            .map(|(a, b)| float_max(a, *b))
            .collect();
        (min, max)
    });

    let range: Vec<(f64, f64)> = min.into_iter().zip(max.into_iter()).collect();

    let scaled = evaluations.into_iter().map(|eval| {
        eval.into_iter()
            .zip(range.iter())
            .map(|(score, (min, max))| {
                let range = max - min;
                (score - min) / if range != 0. { range } else { 1. }
            })
            .collect()
    });

    let ranks: Vec<f64> = scaled
        .map(|eval: Vec<f64>| {
            eval.into_iter()
                .zip(features.iter())
                .map(|(x, feature)| (feature.weight)(x))
                .sum()
        })
        .collect();

    commands
        .into_iter()
        .zip(ranks)
        .map(|(command, rank)| RankedCommand { command, rank })
        .collect()
}
