use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution10;
impl Solution10 {
    fn look_and_say(value: &str) -> String {
        value
            .chars()
            .group_by(|c| *c)
            .into_iter()
            .map(|(v, g)| format!("{}{v}", g.count()))
            .join("")
    }
}

impl Solution for Solution10 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(6),
            ProblemResult::USize(360154),
            ProblemResult::USize(6),
            ProblemResult::USize(5103798),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let iterations = if is_sample { 5 } else { 40 };
        let final_str = (0..iterations).fold(input.string(), |value, _| Self::look_and_say(&value));
        final_str.len().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        // Computation by now takes some seconds, could use
        // https://mathworld.wolfram.com/CosmologicalTheorem.html and fast exponentiation on matrices to solve
        let iterations = if is_sample { 5 } else { 50 };
        let final_str = (0..iterations).fold(input.string(), |value, _| Self::look_and_say(&value));
        final_str.len().to_result()
    }
}
