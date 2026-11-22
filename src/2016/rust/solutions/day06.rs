use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input
        .lines()
        .into_iter()
        .flat_map(|l| l.chars().enumerate().collect_vec())
        .into_group_map()
        .into_iter()
        .map(|(pos, chars)| {
            (
                pos,
                chars
                    .into_iter()
                    .counts()
                    .into_iter()
                    .max_by_key(|(_, count)| *count)
                    .unwrap()
                    .0,
            )
        })
        .sorted_by_key(|(pos, _)| *pos)
        .map(|(_, c)| c)
        .join("")
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input
        .lines()
        .into_iter()
        .flat_map(|l| l.chars().enumerate().collect_vec())
        .into_group_map()
        .into_iter()
        .map(|(pos, chars)| {
            (
                pos,
                chars
                    .into_iter()
                    .counts()
                    .into_iter()
                    .min_by_key(|(_, count)| *count)
                    .unwrap()
                    .0,
            )
        })
        .sorted_by_key(|(pos, _)| *pos)
        .map(|(_, c)| c)
        .join("")
        .into()
}
