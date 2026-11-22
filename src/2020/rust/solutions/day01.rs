use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let entries = parse(input);

    entries
        .into_iter()
        .combinations(2)
        .find(|pair| pair.iter().sum::<u32>() == 2020)
        .map(|pair| pair.iter().product::<u32>())
        .unwrap()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let entries = parse(input);

    entries
        .into_iter()
        .combinations(3)
        .find(|pair| pair.iter().sum::<u32>() == 2020)
        .map(|pair| pair.iter().product::<u32>())
        .unwrap()
        .into()
}

fn parse(input: ProblemInput) -> Vec<u32> {
    input.lines().into_iter().parsed().collect()
}
