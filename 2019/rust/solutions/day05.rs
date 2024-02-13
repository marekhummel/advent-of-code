use aoc_lib::solution::Solution;
use aoc_lib::specific::intcode::Program;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution05;
impl Solution05 {}

impl Solution for Solution05 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let mut program = Program::init(&input.string());
        program.input.push_back(1);

        program.execute();
        program.output.iter().last().unwrap().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let mut program = Program::init(&input.string());
        program.input.push_back(if is_sample { 20 } else { 5 });

        program.execute();
        program.output.iter().last().unwrap().to_result()
    }
}
