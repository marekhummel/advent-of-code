use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::{Itertools, MinMaxResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let spreadsheet = parse(input);

    spreadsheet
        .into_iter()
        .map(|row| {
            let MinMaxResult::MinMax(x, y) = row.iter().minmax() else {
                return 0;
            };
            y - x
        })
        .sum::<u32>()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let spreadsheet = parse(input);

    spreadsheet
        .into_iter()
        .map(|row| {
            let (a, b) = row
                .into_iter()
                .array_combinations()
                .map(|[a, b]| (a.max(b), a.min(b)))
                .find(|(a, b)| a % b == 0)
                .unwrap();
            a / b
        })
        .sum::<u32>()
        .into()
}

fn parse(input: ProblemInput) -> Vec<Vec<u32>> {
    input
        .lines()
        .into_iter()
        .map(|l| l.split_whitespace().parsed().collect_vec())
        .collect_vec()
}
