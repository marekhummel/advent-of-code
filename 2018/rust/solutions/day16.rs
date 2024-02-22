use std::collections::{HashMap, HashSet};

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::wristdevice::{Instruction, Op};
use itertools::Itertools;

type Registers = Vec<usize>;
type NumInst = (usize, usize, usize, usize);
type Sample = (Registers, Registers, NumInst);

pub struct Solution16;
impl Solution16 {
    fn parse(input: ProblemInput) -> (Vec<Sample>, Vec<NumInst>) {
        let samples = input
            .lines()
            .into_iter()
            .chunks(4)
            .into_iter()
            .map_while(|chunk| {
                let lines = chunk.collect_vec();
                let before = lines[0]
                    .trim_start_matches("Before: [")
                    .trim_end_matches(']')
                    .split(',')
                    .parsed()
                    .collect_vec();
                let after = lines[2]
                    .trim_start_matches("After:  [")
                    .trim_end_matches(']')
                    .split(',')
                    .parsed()
                    .collect_vec();
                let instruction = lines[1].split_whitespace().parsed().collect_vec();

                if before.is_empty() || after.is_empty() || instruction.is_empty() {
                    None
                } else {
                    Some((before, after, instruction.into_iter().collect_tuple().unwrap()))
                }
            })
            .collect_vec();

        let program = input
            .lines()
            .into_iter()
            .skip(samples.len() * 4)
            .skip_while(|l| l.is_empty())
            .map(|l| l.split_whitespace().parsed().collect_tuple().unwrap())
            .collect_vec();

        (samples, program)
    }

    fn find_opcode_mapping(mut op_code_mappings: HashMap<usize, HashSet<Op>>) -> HashMap<usize, Op> {
        let mut progress = true;
        while progress {
            progress = false;
            for n in 0..16 {
                if op_code_mappings[&n].len() == 1 {
                    let found_op_code = *op_code_mappings[&n].iter().next().unwrap();
                    for n2 in 0..16 {
                        if n != n2 {
                            progress |= op_code_mappings.get_mut(&n2).unwrap().remove(&found_op_code);
                        }
                    }
                }
            }
        }

        assert!(op_code_mappings.values().all(|ops| ops.len() == 1));

        op_code_mappings
            .into_iter()
            .map(|(n, set)| (n, *set.iter().next().unwrap()))
            .collect()
    }
}

impl Solution for Solution16 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I32(1),
            ProblemResult::I32(570),
            ProblemResult::NoSample,
            ProblemResult::USize(503),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (samples, _) = Self::parse(input);

        let mut solution = 0;
        for (before, after, inst) in samples {
            let (_, a, b, c) = inst;
            let match_count = Op::iter()
                .filter(|&opcode| {
                    let mut registers = before.clone();
                    let instruction = Instruction { opcode, a, b, c };
                    instruction.execute(&mut registers);

                    registers == after
                })
                .count();

            if match_count >= 3 {
                solution += 1;
            }
        }

        solution.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let (samples, program) = Self::parse(input);

        // Run through samples and find possible opcodes per number
        let mut op_code_mappings: HashMap<usize, HashSet<_>> =
            (0usize..16).map(|n| (n, HashSet::from_iter(Op::iter()))).collect();
        for (before, after, inst) in samples {
            let (n, a, b, c) = inst;
            let matches: HashSet<_> = Op::iter()
                .filter(|&opcode| {
                    let mut registers = before.clone();
                    let instruction = Instruction { opcode, a, b, c };
                    instruction.execute(&mut registers);

                    registers == after
                })
                .collect();

            op_code_mappings.get_mut(&n).unwrap().retain(|op| matches.contains(op));
        }

        // Create and run program
        let op_code_map = Self::find_opcode_mapping(op_code_mappings);
        let mut registers = vec![0; 4];
        for (op, a, b, c) in program {
            let opcode = op_code_map[&op];
            let inst = Instruction { opcode, a, b, c };
            inst.execute(&mut registers);
        }

        registers[0].to_result()
    }
}
