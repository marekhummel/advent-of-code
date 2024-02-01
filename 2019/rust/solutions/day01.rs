use std::collections::VecDeque;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution01;
impl Solution01 {}

impl Solution for Solution01 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        input
            .lines()
            .into_iter()
            .parsed::<i32>()
            .map(|m| m / 3 - 2)
            .sum::<i32>()
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut masses = VecDeque::from_iter(input.lines().into_iter().parsed::<i32>());

        let mut total_fuel = 0;
        while let Some(m) = masses.pop_front() {
            let fuel = (m / 3 - 2).max(0);
            total_fuel += fuel;
            if fuel > 0 {
                masses.push_back(fuel);
            }
        }

        total_fuel.into_some()
    }
}
