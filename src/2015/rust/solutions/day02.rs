use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    parse(input)
        .into_iter()
        .map(|(l, w, h)| (l * w, l * h, w * h))
        .map(|(a, b, c)| 2 * a + 2 * b + 2 * c + a.min(b.min(c)))
        .sum::<u32>()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    parse(input)
        .into_iter()
        .map(|(l, w, h)| (l + w, l + h, w + h, l * w * h))
        .map(|(a, b, c, v)| 2 * a.min(b.min(c)) + v)
        .sum::<u32>()
        .into()
}

fn parse(input: ProblemInput) -> Vec<(u32, u32, u32)> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            line.split('x')
                .map(|d| d.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}
