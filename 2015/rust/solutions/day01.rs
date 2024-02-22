use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
pub struct Solution01;

impl Solution01 {}

impl Solution for Solution01 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        input
            .string()
            .chars()
            .map(|c| -(((c as u8 - 40) * 2) as i16 - 1))
            .sum::<i16>()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
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

        (index + 1).to_result()
    }
}
