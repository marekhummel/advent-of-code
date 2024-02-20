use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution01;
impl Solution01 {
    fn parse(input: ProblemInput) -> Vec<u32> {
        input.lines().into_iter().parsed().collect()
    }
}

impl Solution for Solution01 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let entries = Self::parse(input);

        entries
            .into_iter()
            .combinations(2)
            .find(|pair| pair.iter().sum::<u32>() == 2020)
            .map(|pair| pair.iter().product::<u32>())
            .unwrap()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let entries = Self::parse(input);

        entries
            .into_iter()
            .combinations(3)
            .find(|pair| pair.iter().sum::<u32>() == 2020)
            .map(|pair| pair.iter().product::<u32>())
            .unwrap()
            .to_result()
    }
}
