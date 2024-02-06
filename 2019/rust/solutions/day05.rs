use aoc_lib::iterator::ParsedExt;
use aoc_lib::solution::Solution;
use aoc_lib::specific::intcode::Program;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution05;
impl Solution05 {}

impl Solution for Solution05 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        if is_sample {
            return None;
        }

        let intcode = input.string().split(',').parsed().collect();
        let mut program = Program::init(intcode);
        program.input.push_back(1);

        program.execute();
        program.output.iter().last().unwrap().into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let intcode = input.string().split(',').parsed().collect();
        let mut program = Program::init(intcode);
        program.input.push_back(if _is_sample { 20 } else { 5 });

        program.execute();
        program.output.iter().last().unwrap().into_some()
    }
}
