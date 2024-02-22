use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::duet::{Instruction, Program, StepResult, Value};

pub struct Solution18;
impl Solution18 {}

impl Solution for Solution18 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut prog = Program::parse(&input, true);

        loop {
            if let StepResult::Recover(x) = prog.step() {
                return x.to_result();
            }
        }
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut p0 = Program::parse(&input, false);
        let mut p1 = Program::parse(&input, false);

        p0.instructions
            .insert(0, Instruction::Set("p".to_string(), Value::Immediate(0)));
        p1.instructions
            .insert(0, Instruction::Set("p".to_string(), Value::Immediate(1)));

        let mut p1_counter = 0;
        loop {
            // Move
            let r0 = p0.step();
            let r1 = p1.step();

            // Deadlock, both receive
            if r0 == StepResult::Receiving && r1 == StepResult::Receiving {
                break;
            }

            // Update queue of p1, if p0 sent sth
            if let StepResult::Send(v0) = r0 {
                p1.queue.push_back(v0);
            }

            // Update queue of p0, if p1 sent sth
            if let StepResult::Send(v1) = r1 {
                p0.queue.push_back(v1);
                p1_counter += 1;
            }
        }

        p1_counter.to_result()
    }
}
