use std::collections::VecDeque;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input
        .lines()
        .into_iter()
        .parsed::<i32>()
        .map(|m| m / 3 - 2)
        .sum::<i32>()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut masses = VecDeque::from_iter(input.lines().into_iter().parsed::<i32>());

    let mut total_fuel = 0;
    while let Some(m) = masses.pop_front() {
        let fuel = (m / 3 - 2).max(0);
        total_fuel += fuel;
        if fuel > 0 {
            masses.push_back(fuel);
        }
    }

    total_fuel.into()
}
