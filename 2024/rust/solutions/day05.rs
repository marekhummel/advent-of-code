use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution05;
impl Solution05 {
    fn parse(input: ProblemInput) {
        input
            .lines()
            .into_iter()
            .map(|l| {
                //
            })
            .collect()
    }
}

impl Solution for Solution05 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        ProblemResult::Unsolved
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        ProblemResult::Unsolved
    }
}
