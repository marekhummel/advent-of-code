use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution06;
impl Solution06 {}

impl Solution for Solution06 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        input
            .lines()
            .split(|l| l.is_empty())
            .map(|answers| answers.iter().flat_map(|ans| ans.bytes()).unique().count())
            .sum::<usize>()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        input
            .lines()
            .split(|l| l.is_empty())
            .map(|answers| {
                answers
                    .iter()
                    .flat_map(|ans| ans.bytes())
                    .counts()
                    .values()
                    .filter(|count| **count == answers.len())
                    .count()
            })
            .sum::<usize>()
            .to_result()
    }
}
