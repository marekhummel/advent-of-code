use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::intcode::Program;

pub struct Solution09;
impl Solution09 {}

impl Solution for Solution09 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I128(1219070632396864),
            ProblemResult::I128(3765554916),
            ProblemResult::I128(1219070632396864),
            ProblemResult::I128(76642),
        ]
    }

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
