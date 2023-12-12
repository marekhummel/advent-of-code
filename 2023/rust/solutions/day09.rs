use itertools::Itertools;

use crate::solution::{ProblemInput, ProblemResult, Solution};

type Sequence = Vec<i64>;

pub struct Solution09;
impl Solution09 {
    fn parse(&self, input: ProblemInput) -> Vec<Sequence> {
        input
            .into_iter()
            .map(|s| s.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect_vec())
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
    fn get_day(&self) -> u8 {
        9
    }

    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        let sequences = self.parse(input);
        Some(sequences.into_iter().map(|s| self.extrapolate(&s)).sum::<i64>().into())
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        let sequences = self.parse(input);
        Some(
            sequences
                .into_iter()
                .map(|s| self.extrapolate(&s.iter().cloned().rev().collect_vec()))
                .sum::<i64>()
                .into(),
        )
    }
}
