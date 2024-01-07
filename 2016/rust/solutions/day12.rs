use std::collections::HashMap;

use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    CopyImmediate(String, i32),
    Copy(String, String),
    Inc(String),
    Dec(String),
    JumpNotZero(String, isize),
}
pub struct Solution12;
impl Solution12 {
    fn parse(input: ProblemInput) -> Vec<Instruction> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (inst, args) = l.split_once(' ').unwrap();
                match inst {
                    "cpy" => {
                        let (src, reg) = args.split_once(' ').unwrap();
                        if let Ok(value) = src.trim().parse() {
                            Instruction::CopyImmediate(reg.to_string(), value)
                        } else {
                            Instruction::Copy(reg.to_string(), src.trim().to_string())
                        }
                    }
                    "inc" => Instruction::Inc(args.to_string()),
                    "dec" => Instruction::Dec(args.to_string()),
                    "jnz" => {
                        let (reg, offset) = args.split_once(' ').unwrap();
                        Instruction::JumpNotZero(reg.to_string(), offset.trim().parse().unwrap())
                    }
                    _ => panic!("Unknown instruction"),
                }
            })
            .collect_vec()
    }

    fn run_program(program: &[Instruction], registers: &mut HashMap<String, i32>) {
        let mut pc = 0usize;
        while pc < program.len() {
            match &program[pc] {
                Instruction::CopyImmediate(reg, value) => *registers.get_mut(reg).unwrap() = *value,
                Instruction::Copy(trg, src) => *registers.get_mut(trg).unwrap() = *registers.get(src).unwrap(),
                Instruction::Inc(reg) => *registers.get_mut(reg).unwrap() += 1,
                Instruction::Dec(reg) => *registers.get_mut(reg).unwrap() -= 1,
                Instruction::JumpNotZero(reg, offset) => {
                    let value = if let Ok(imm) = reg.trim().parse() {
                        imm
                    } else {
                        registers[reg]
                    };
                    if value != 0 {
                        pc = (pc as isize + offset - 1) as usize
                    }
                }
            }
            pc += 1;
        }
    }

    fn init_registers(registers: Vec<&str>) -> HashMap<String, i32> {
        registers.into_iter().map(|r| (r.to_string(), 0)).collect()
    }
}

impl Solution for Solution12 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let program = Self::parse(input);

        let mut registers = Self::init_registers(vec!["a", "b", "c", "d"]);
        Self::run_program(&program, &mut registers);

        registers["a"].into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut program = Self::parse(input);
        program.insert(0, Instruction::CopyImmediate("c".to_string(), 1));

        let mut registers = Self::init_registers(vec!["a", "b", "c", "d"]);
        Self::run_program(&program, &mut registers);

        registers["a"].into_some()
    }
}
