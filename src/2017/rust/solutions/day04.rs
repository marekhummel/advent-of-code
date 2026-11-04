use aoc_lib::iterator::IsUniqueExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution04;
impl Solution04 {}

impl Solution for Solution04 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        input
            .lines()
            .into_iter()
            .filter(|l| l.split_whitespace().is_unique())
            .count()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        input
            .lines()
            .into_iter()
            .filter(|l| l.split_whitespace().map(|x| x.chars().sorted().join("")).is_unique())
            .count()
            .to_result()
    }
}
