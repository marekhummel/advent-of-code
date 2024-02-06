use aoc_lib::solution::Solution;
use aoc_lib::specific::intcode::Program;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution02;
impl Solution02 {}

impl Solution for Solution02 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let mut program = Program::init(&input.string());
        if !is_sample {
            *program.memory.get_mut(&1).unwrap() = 12;
            *program.memory.get_mut(&2).unwrap() = 2;
        }

        program.execute();
        program.memory[&0].into_some()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let intcode = input.string();

        if is_sample {
            return None;
        }

        for noun in 0..100 {
            for verb in 0..100 {
                let mut program = Program::init(&intcode);
                *program.memory.get_mut(&1).unwrap() = noun;
                *program.memory.get_mut(&2).unwrap() = verb;

                program.execute();

                if program.memory[&0] == 19690720 {
                    return (noun * 100 + verb).into_some();
                }
            }
        }

        unreachable!()
    }
}
