use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, is_sample: bool) -> ProblemResult {
    let total_rows = if is_sample { 10 } else { 40 };
    let mut row = input.string().chars().map(|c| c == '.').collect_vec();
    let mut safe_tiles = 0;
    for _ in 0..total_rows {
        safe_tiles += row.iter().filter(|s| **s).count();
        row = next_row(row);
    }

    safe_tiles.into()
}

pub fn part2(input: ProblemInput, is_sample: bool) -> ProblemResult {
    let total_rows = if is_sample { 10 } else { 400000 };
    let mut row = input.string().chars().map(|c| c == '.').collect_vec();
    let mut safe_tiles = 0;
    for _ in 0..total_rows {
        safe_tiles += row.iter().filter(|s| **s).count();
        row = next_row(row);
    }

    safe_tiles.into()
}

fn next_row(row: Vec<bool>) -> Vec<bool> {
    let extended_row = [vec![true], row, vec![true]].concat();
    extended_row
        .into_iter()
        .tuple_windows()
        .map(|(a, _, c)| a && c || !a && !c) // evident from looking at the bit table, equal to a == c
        .collect_vec()
}
