use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use aoc_lib::specific::intcode::Program;

pub fn part1(input: ProblemInput, is_sample: bool) -> ProblemResult {
    if is_sample {
        return ProblemResult::NoSample;
    }

    let mut program = Program::init(&input.string());
    program.input.push_back(1);

    program.execute();
    program.output.iter().last().unwrap().into()
}

pub fn part2(input: ProblemInput, is_sample: bool) -> ProblemResult {
    let mut program = Program::init(&input.string());
    program.input.push_back(if is_sample { 20 } else { 5 });

    program.execute();
    program.output.iter().last().unwrap().into()
}
