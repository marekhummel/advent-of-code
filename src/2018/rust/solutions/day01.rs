use std::collections::HashSet;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input.lines().into_iter().parsed::<i32>().sum::<i32>().into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let changes = input.lines().into_iter().parsed::<i32>().collect_vec();
    let mut seen = HashSet::new();
    let mut freq = 0;

    for c in changes.into_iter().cycle() {
        freq += c;
        if seen.contains(&freq) {
            break;
        }
        seen.insert(freq);
    }

    freq.into()
}
