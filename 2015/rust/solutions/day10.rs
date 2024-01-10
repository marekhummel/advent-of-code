use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
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
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let iterations = if is_sample { 5 } else { 40 };
        let final_str = (0..iterations).fold(input.string(), |value, _| Self::look_and_say(&value));
        final_str.len().into_some()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        // Computation by now takes some seconds, could use
        // https://mathworld.wolfram.com/CosmologicalTheorem.html and fast exponentiation on matrices to solve
        let iterations = if is_sample { 5 } else { 50 };
        let final_str = (0..iterations).fold(input.string(), |value, _| Self::look_and_say(&value));
        final_str.len().into_some()
    }
}
