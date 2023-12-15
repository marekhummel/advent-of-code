#![allow(unused_variables)]
use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult};
pub struct Solution01;

impl Solution01 {}

impl Solution for Solution01 {
    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        Some(
            input
                .join("")
                .chars()
                .map(|c| -(((c as u8 - 40) * 2) as i16 - 1))
                .sum::<i16>()
                .into(),
        )
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        let index = input
            .join("")
            .chars()
            .map(|c| -(((c as u8 - 40) * 2) as i16 - 1))
            .scan(0, |floor, dir| {
                *floor += dir;
                Some(*floor)
            })
            .take_while(|floor| *floor != -1)
            .count();

        Some((index + 1).into())
    }
}
