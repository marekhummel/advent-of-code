use std::collections::{HashMap, HashSet};

use aoc_lib::solution::Solution;
use aoc_lib::specific::assembunny::{Instruction, Program, Value};
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

pub struct Solution25;
impl Solution25 {}

impl Solution for Solution25 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        if _is_sample {
            return None;
        }

        let mut program = Program::from_input(input);
        println!("{}", program.optimize_multiplication());
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
                        return a.into_some();
                    }
                }

                history.insert((program.pc, reg_snap), last_output);
                program.step();
            }
        }

        unreachable!()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        // No part 2
        None
    }
}
