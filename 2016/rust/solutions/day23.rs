use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

use aoc_lib::specific::assembunny::{Instruction, Program, Value};

pub struct Solution23;
impl Solution23 {}

impl Solution for Solution23 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut program = Program::from_input(input);
        program
            .instructions
            .insert(0, Instruction::Copy(Value::Immediate(7), "a".to_string()));

        program.run();
        program.registers["a"].into_some()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let mut program = Program::from_input(input);
        program
            .instructions
            .insert(0, Instruction::Copy(Value::Immediate(12), "a".to_string()));

        let optimized = program.optimize_multiplication();
        assert_eq!(optimized, !is_sample, "Need optimization for real input in v2");
        program.run();
        program.registers["a"].into_some()
    }
}
