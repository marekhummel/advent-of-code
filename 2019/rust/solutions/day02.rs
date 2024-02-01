use aoc_lib::iterator::ParsedExt;
use aoc_lib::solution::Solution;
use aoc_lib::specific::intcode::Program;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution02;
impl Solution02 {
    fn parse(input: ProblemInput) -> Vec<u32> {
        input.string().split(',').parsed().collect()
    }
}

impl Solution for Solution02 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let mut intcode = Self::parse(input);

        if !is_sample {
            *intcode.get_mut(1).unwrap() = 12;
            *intcode.get_mut(2).unwrap() = 2;
        }

        let mut program = Program { intcode };
        program.execute();
        program.intcode[0].into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let intcode = Self::parse(input);

        if _is_sample {
            return None;
        }

        for noun in 0..100 {
            for verb in 0..100 {
                let mut program = Program {
                    intcode: intcode.clone(),
                };
                *program.intcode.get_mut(1).unwrap() = noun;
                *program.intcode.get_mut(2).unwrap() = verb;

                program.execute();

                if program.intcode[0] == 19690720 {
                    return (noun * 100 + verb).into_some();
                }
            }
        }

        unreachable!()
    }
}
