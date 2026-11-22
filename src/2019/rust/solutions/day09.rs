use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use aoc_lib::specific::intcode::Program;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut program = Program::init(&input.string());

    program.input.push_back(1);
    program.execute();
    program.output[0].into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut program = Program::init(&input.string());

    program.input.push_back(2);
    program.execute();
    program.output[0].into()
}
