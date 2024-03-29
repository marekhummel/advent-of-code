use std::collections::HashMap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::assembunny::{Instruction, Program, Value};
use itertools::Itertools;

pub struct Solution25;
impl Solution25 {}

impl Solution for Solution25 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::NoSample,
            ProblemResult::I32(192),
            ProblemResult::NoPartTwo,
            ProblemResult::NoPartTwo,
        ]
    }

    // Still runs for a couple of seconds, even after code optimization
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let mut program = Program::from_input(input);
        assert!(program.optimize_multiplication(), "Expected optimization");
        program
            .instructions
            .insert(0, Instruction::Copy(Value::Immediate(0), "a".to_string()));

        for a in 0.. {
            program.instructions[0] = Instruction::Copy(Value::Immediate(a), "a".to_string());
            program.reset();

            let mut history = HashMap::new();

            loop {
                // Not running forever
                if program.is_completed() {
                    break;
                }

                // Not a clock signal
                if !program.output.iter().tuples().all(|(c0, c1)| *c0 == 0 && *c1 == 1) {
                    break;
                }

                // Program in same state as once before
                let reg_snap = program
                    .registers
                    .iter()
                    .sorted_by_key(|(reg, _)| *reg)
                    .map(|(_, val)| *val)
                    .collect_vec();
                let last_output = program.output.last().copied();

                if let Some(history_last_output) = history.get(&(program.pc, reg_snap.clone())) {
                    if history_last_output == &last_output {
                        return a.to_result();
                    }
                }

                history.insert((program.pc, reg_snap), last_output);
                program.step();
            }
        }

        unreachable!()
    }

    fn solve_version02(&self, _input: ProblemInput, _is_sample: bool) -> ProblemResult {
        // No part 2
        ProblemResult::NoPartTwo
    }
}
