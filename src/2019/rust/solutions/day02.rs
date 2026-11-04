use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::intcode::Program;

pub struct Solution02;
impl Solution02 {}

impl Solution for Solution02 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let mut program = Program::init(&input.string());
        if !is_sample {
            *program.memory.get_mut(&1).unwrap() = 12;
            *program.memory.get_mut(&2).unwrap() = 2;
        }

        program.execute();
        program.memory[&0].to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let intcode = input.string();

        if is_sample {
            return ProblemResult::NoSample;
        }

        for noun in 0..100 {
            for verb in 0..100 {
                let mut program = Program::init(&intcode);
                *program.memory.get_mut(&1).unwrap() = noun;
                *program.memory.get_mut(&2).unwrap() = verb;

                program.execute();

                if program.memory[&0] == 19690720 {
                    return (noun * 100 + verb).to_result();
                }
            }
        }

        unreachable!()
    }
}
