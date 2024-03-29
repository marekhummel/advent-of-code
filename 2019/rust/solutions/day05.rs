use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::intcode::Program;

pub struct Solution05;
impl Solution05 {}

impl Solution for Solution05 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::NoSample,
            ProblemResult::I128(15259545),
            ProblemResult::I128(1001),
            ProblemResult::I128(7616021),
        ]
    }

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
