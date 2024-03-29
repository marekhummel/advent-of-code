use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

use aoc_lib::specific::assembunny::{Instruction, Program, Value};

pub struct Solution23;
impl Solution23 {}

impl Solution for Solution23 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I32(3),
            ProblemResult::I32(11760),
            ProblemResult::I32(3),
            ProblemResult::I32(479008320),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut program = Program::from_input(input);
        program
            .instructions
            .insert(0, Instruction::Copy(Value::Immediate(7), "a".to_string()));

        program.run();
        program.registers["a"].to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let mut program = Program::from_input(input);
        program
            .instructions
            .insert(0, Instruction::Copy(Value::Immediate(12), "a".to_string()));

        let optimized = program.optimize_multiplication();
        assert_eq!(optimized, !is_sample, "Need optimization for real input in v2");
        program.run();
        program.registers["a"].to_result()
    }
}
