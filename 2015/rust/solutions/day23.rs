use std::collections::HashMap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    Half(String),
    Triple(String),
    Inc(String),
    Jump(isize),
    JumpIfEven(String, isize),
    JumpIfOne(String, isize),
}

pub struct Solution23;
impl Solution23 {
    fn parse(input: ProblemInput) -> Vec<Instruction> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (inst, args) = l.split_once(' ').unwrap();
                match inst {
                    "hlf" => Instruction::Half(args.to_string()),
                    "tpl" => Instruction::Triple(args.to_string()),
                    "inc" => Instruction::Inc(args.to_string()),
                    "jmp" => Instruction::Jump(args.trim().parse().unwrap()),
                    "jie" => {
                        let (reg, offset) = args.split_once(',').unwrap();
                        Instruction::JumpIfEven(reg.to_string(), offset.trim().parse().unwrap())
                    }
                    "jio" => {
                        let (reg, offset) = args.split_once(',').unwrap();
                        Instruction::JumpIfOne(reg.to_string(), offset.trim().parse().unwrap())
                    }
                    _ => panic!("Unknown instruction"),
                }
            })
            .collect_vec()
    }

    fn run_program(program: &[Instruction], registers: &mut HashMap<String, u32>) {
        let mut pc = 0usize;
        while pc < program.len() {
            match &program[pc] {
                Instruction::Half(reg) => *registers.get_mut(reg).unwrap() /= 2,
                Instruction::Triple(reg) => *registers.get_mut(reg).unwrap() *= 3,
                Instruction::Inc(reg) => *registers.get_mut(reg).unwrap() += 1,
                Instruction::Jump(offset) => pc = (pc as isize + offset - 1) as usize,
                Instruction::JumpIfEven(reg, offset) => {
                    if registers[reg] & 1 == 0 {
                        pc = (pc as isize + offset - 1) as usize
                    }
                }
                Instruction::JumpIfOne(reg, offset) => {
                    if registers[reg] == 1 {
                        pc = (pc as isize + offset - 1) as usize
                    }
                }
            }

            pc += 1;
        }
    }
}

impl Solution for Solution23 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let program = Self::parse(input);
        let mut registers = HashMap::from([(String::from("a"), 0u32), (String::from("b"), 0u32)]);
        Self::run_program(&program, &mut registers);

        let target = if is_sample { "a" } else { "b" };
        registers[target].to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let program = Self::parse(input);
        let mut registers = HashMap::from([(String::from("a"), 1u32), (String::from("b"), 0u32)]);
        Self::run_program(&program, &mut registers);

        let target = if is_sample { "a" } else { "b" };
        registers[target].to_result()
    }
}
