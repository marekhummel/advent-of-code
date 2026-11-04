use aoc_lib::iterator::ParsedExt;
use itertools::Itertools;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

type Sequence = Vec<i64>;

pub struct Solution09;
impl Solution09 {
    fn parse(&self, input: ProblemInput) -> Vec<Sequence> {
        input
            .lines()
            .into_iter()
            .map(|s| s.split_whitespace().parsed().collect_vec())
            .collect_vec()
    }

    fn diff(&self, sequence: &Sequence) -> Sequence {
        sequence
            .iter()
            .skip(1)
            .zip(sequence.iter())
            .map(|(c, p)| c - p)
            .collect_vec()
    }

    fn extrapolate(&self, seq: &Sequence) -> i64 {
        let mut seq_derivs = Vec::from([seq.iter().cloned().collect_vec()]);
        let mut diffs = seq_derivs.last().unwrap();
        while !diffs.iter().all(|d| *d == 0) {
            let next_deriv = self.diff(diffs);
            seq_derivs.push(next_deriv);
            diffs = seq_derivs.last().unwrap();
        }

        seq_derivs.iter().map(|d| d.last().unwrap()).sum::<i64>()
    }
}

impl Solution for Solution09 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let sequences = self.parse(input);
        sequences
            .into_iter()
            .map(|s| self.extrapolate(&s))
            .sum::<i64>()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let sequences = self.parse(input);
        sequences
            .into_iter()
            .map(|s| self.extrapolate(&s.iter().cloned().rev().collect_vec()))
            .sum::<i64>()
            .to_result()
    }
}
