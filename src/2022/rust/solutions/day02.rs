use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

#[allow(clippy::identity_op)]
pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let strategies = parse(input);

    let score: u32 = strategies
        .into_iter()
        .map(|strat| match strat {
            ('A', 'X') => 1 + 3,
            ('A', 'Y') => 2 + 6,
            ('A', 'Z') => 3 + 0,
            ('B', 'X') => 1 + 0,
            ('B', 'Y') => 2 + 3,
            ('B', 'Z') => 3 + 6,
            ('C', 'X') => 1 + 6,
            ('C', 'Y') => 2 + 0,
            ('C', 'Z') => 3 + 3,
            _ => unreachable!(),
        })
        .sum();

    score.into()
}

#[allow(clippy::identity_op)]
pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let strategies = parse(input);

    let score: u32 = strategies
        .into_iter()
        .map(|strat| match strat {
            ('A', 'X') => 0 + 3,
            ('A', 'Y') => 3 + 1,
            ('A', 'Z') => 6 + 2,
            ('B', 'X') => 0 + 1,
            ('B', 'Y') => 3 + 2,
            ('B', 'Z') => 6 + 3,
            ('C', 'X') => 0 + 2,
            ('C', 'Y') => 3 + 3,
            ('C', 'Z') => 6 + 1,
            _ => unreachable!(),
        })
        .sum();

    score.into()
}

fn parse(input: ProblemInput) -> Vec<(char, char)> {
    input
        .lines()
        .into_iter()
        .map(|l| {
            let chars = l.chars().collect_vec();
            (chars[0], chars[2])
        })
        .collect()
}
