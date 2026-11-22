use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, is_sample: bool) -> ProblemResult {
    let iterations = if is_sample { 5 } else { 40 };
    let final_str = (0..iterations).fold(input.string(), |value, _| look_and_say(&value));
    final_str.len().into()
}

pub fn part2(input: ProblemInput, is_sample: bool) -> ProblemResult {
    // Computation by now takes some seconds, could use
    // https://mathworld.wolfram.com/CosmologicalTheorem.html and fast exponentiation on matrices to solve
    let iterations = if is_sample { 5 } else { 50 };
    let final_str = (0..iterations).fold(input.string(), |value, _| look_and_say(&value));
    final_str.len().into()
}

fn look_and_say(value: &str) -> String {
    value
        .chars()
        .chunk_by(|c| *c)
        .into_iter()
        .map(|(v, g)| format!("{}{v}", g.count()))
        .join("")
}
