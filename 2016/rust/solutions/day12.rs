use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

use aoc_lib::specific::assembunny::{Instruction, Program, Value};

pub struct Solution12;
impl Solution12 {}

impl Solution for Solution12 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut program = Program::from_input(input);

        program.run();
        program.registers["a"].into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut program = Program::from_input(input);
        program
            .instructions
            .insert(0, Instruction::Copy(Value::Immediate(1), "c".to_string()));

        program.run();
        program.registers["a"].into_some()
    }
}
