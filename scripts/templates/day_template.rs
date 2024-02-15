use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult};

pub struct SolutionXX;
impl SolutionXX {
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

impl Solution for SolutionXX {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        ProblemResult::Unsolved
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        ProblemResult::Unsolved
    }
}
