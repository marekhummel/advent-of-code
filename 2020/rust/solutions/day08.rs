use std::collections::HashSet;
use std::usize;

use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution08;
impl Solution08 {
    fn run_instruction(inst: &str, acc: &mut i64, pc: &mut usize) {
        let (op, arg) = inst.split_once(' ').unwrap();
        match op {
            "acc" => *acc += arg.parse::<i64>().unwrap(),
            "jmp" => *pc = (*pc as isize + arg.parse::<isize>().unwrap() - 1) as usize,
            "nop" => (),
            _ => unreachable!(),
        }

        *pc += 1;
    }
}

impl Solution for Solution08 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let boot_code = input.lines();
        let mut acc = 0;
        let mut pc: usize = 0;

        let mut executed = HashSet::new();
        while !executed.contains(&pc) {
            executed.insert(pc);
            Self::run_instruction(&boot_code[pc], &mut acc, &mut pc);
        }

        acc.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        for corrupted in 0.. {
            let mut boot_code = input.lines();
            let corrupted_line = &boot_code[corrupted];
            match &corrupted_line[..3] {
                "acc" => continue,
                "jmp" => *boot_code.get_mut(corrupted).unwrap() = corrupted_line.replace("jmp", "nop"),
                "nop" => *boot_code.get_mut(corrupted).unwrap() = corrupted_line.replace("nop", "jmp"),
                _ => unreachable!(),
            }

            let mut acc = 0;
            let mut pc: usize = 0;
            let mut executed = HashSet::new();
            while !executed.contains(&pc) {
                executed.insert(pc);
                Self::run_instruction(&boot_code[pc], &mut acc, &mut pc);

                if pc >= boot_code.len() {
                    return acc.to_result();
                }
            }
        }

        unreachable!()
    }
}
