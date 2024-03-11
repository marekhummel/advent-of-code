use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution01;
impl Solution01 {
    fn parse(input: ProblemInput) -> impl Iterator<Item = u32> {
        let lines = input.lines();
        let elves = lines.split(|l| l.is_empty());
        elves.map(|elf| elf.iter().parsed::<u32>().sum()).sorted().rev()
    }
}

impl Solution for Solution01 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(24000),
            ProblemResult::U32(69883),
            ProblemResult::U32(45000),
            ProblemResult::U32(207576),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let elves = Self::parse(input).next().unwrap();
        elves.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let elves = Self::parse(input).take(3).sum::<u32>();
        elves.to_result()
    }
}
