use aoc_lib::iterator::ParsedExt;
use itertools::Itertools;

use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let sequences = parse(input);
    sequences
        .into_iter()
        .map(|s| extrapolate(&s))
        .sum::<i64>()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let sequences = parse(input);
    sequences
        .into_iter()
        .map(|s| extrapolate(&s.iter().cloned().rev().collect_vec()))
        .sum::<i64>()
        .into()
}

type Sequence = Vec<i64>;

fn parse(input: ProblemInput) -> Vec<Sequence> {
    input
        .lines()
        .into_iter()
        .map(|s| s.split_whitespace().parsed().collect_vec())
        .collect_vec()
}

fn diff(sequence: &Sequence) -> Sequence {
    sequence
        .iter()
        .skip(1)
        .zip(sequence.iter())
        .map(|(c, p)| c - p)
        .collect_vec()
}

fn extrapolate(seq: &Sequence) -> i64 {
    let mut seq_derivs = Vec::from([seq.iter().cloned().collect_vec()]);
    let mut diffs = seq_derivs.last().unwrap();
    while !diffs.iter().all(|d| *d == 0) {
        let next_deriv = diff(diffs);
        seq_derivs.push(next_deriv);
        diffs = seq_derivs.last().unwrap();
    }

    seq_derivs.iter().map(|d| d.last().unwrap()).sum::<i64>()
}
