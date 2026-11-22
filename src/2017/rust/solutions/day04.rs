use aoc_lib::iterator::IsUniqueExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input
        .lines()
        .into_iter()
        .filter(|l| l.split_whitespace().is_unique())
        .count()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input
        .lines()
        .into_iter()
        .filter(|l| l.split_whitespace().map(|x| x.chars().sorted().join("")).is_unique())
        .count()
        .into()
}
