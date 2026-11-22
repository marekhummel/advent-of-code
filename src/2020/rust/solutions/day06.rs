use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input
        .lines()
        .split(|l| l.is_empty())
        .map(|answers| answers.iter().flat_map(|ans| ans.bytes()).unique().count())
        .sum::<usize>()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
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
        .into()
}
