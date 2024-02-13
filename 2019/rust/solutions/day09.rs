use aoc_lib::solution::Solution;
use aoc_lib::specific::intcode::Program;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution09;
impl Solution09 {}

impl Solution for Solution09 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut program = Program::init(&input.string());

        program.input.push_back(1);
        program.execute();
        program.output[0].to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut program = Program::init(&input.string());

        program.input.push_back(2);
        program.execute();
        program.output[0].to_result()
    }
}
