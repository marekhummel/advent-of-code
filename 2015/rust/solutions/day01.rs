#![allow(unused_variables)]
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
pub struct Solution01;

impl Solution01 {}

impl Solution for Solution01 {
    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        input
            .string()
            .chars()
            .map(|c| -(((c as u8 - 40) * 2) as i16 - 1))
            .sum::<i16>()
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        let index = input
            .string()
            .chars()
            .map(|c| -(((c as u8 - 40) * 2) as i16 - 1))
            .scan(0, |floor, dir| {
                *floor += dir;
                Some(*floor)
            })
            .take_while(|floor| *floor != -1)
            .count();

        (index + 1).into_some()
    }
}
