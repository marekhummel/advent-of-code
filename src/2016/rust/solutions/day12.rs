use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

use aoc_lib::specific::assembunny::{Instruction, Program, Value};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut program = Program::from_input(input);

    program.run();
    program.registers["a"].into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut program = Program::from_input(input);
    program
        .instructions
        .insert(0, Instruction::Copy(Value::Immediate(1), "c".to_string()));

    program.run();
    program.registers["a"].into()
}
