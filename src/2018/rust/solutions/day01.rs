use std::collections::HashSet;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution01;
impl Solution01 {}

impl Solution for Solution01 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I32(3),
            ProblemResult::I32(513),
            ProblemResult::I32(2),
            ProblemResult::I32(287),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        input.lines().into_iter().parsed::<i32>().sum::<i32>().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
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

        freq.to_result()
    }
}
