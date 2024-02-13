use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult};

pub struct Solution20;
impl Solution20 {
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

impl Solution for Solution20 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        None
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        None
    }
}
