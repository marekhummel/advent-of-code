use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let elves = parse(input).next().unwrap();
    elves.into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let elves = parse(input).take(3).sum::<u32>();
    elves.into()
}

fn parse(input: ProblemInput) -> impl Iterator<Item = u32> {
    let lines = input.lines();
    let elves = lines.split(|l| l.is_empty());
    elves.map(|elf| elf.iter().parsed::<u32>().sum()).sorted().rev()
}
