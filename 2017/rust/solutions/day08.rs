use std::collections::HashMap;

use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

#[derive(Debug)]
struct Condition {
    register: String,
    value: i32,
    operator: String,
}

#[derive(Debug)]
struct Instruction {
    register: String,
    value: i32,
    condition: Condition,
}

pub struct Solution08;
impl Solution08 {
    fn parse(input: ProblemInput) -> Vec<Instruction> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let words = l.split_whitespace().collect_vec();
                let value_factor = if words[1] == "inc" { 1 } else { -1 };
                let op_value = words[2].parse::<i32>().unwrap() * value_factor;

                Instruction {
                    register: words[0].to_string(),
                    value: op_value,
                    condition: Condition {
                        register: words[4].to_string(),
                        value: words[6].parse().unwrap(),
                        operator: words[5].to_string(),
                    },
                }
            })
            .collect_vec()
    }

    fn step(inst: &Instruction, registers: &mut HashMap<String, i32>) {
        let reg = *registers.get(&inst.condition.register).unwrap_or(&0);
        let val = inst.condition.value;
        let condition_eval = match inst.condition.operator.as_str() {
            "==" => reg == val,
            "!=" => reg != val,
            "<=" => reg <= val,
            ">=" => reg >= val,
            "<" => reg < val,
            ">" => reg > val,
            _ => unreachable!(),
        };
        if condition_eval {
            *registers.entry(inst.register.clone()).or_insert(0) += inst.value;
        }
    }
}

impl Solution for Solution08 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let instructions = Self::parse(input);
        let mut registers = HashMap::new();

        for inst in instructions {
            Self::step(&inst, &mut registers);
        }
        registers.values().max().unwrap().into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let instructions = Self::parse(input);
        let mut registers = HashMap::new();

        let mut highest = i32::MIN;
        for inst in instructions {
            Self::step(&inst, &mut registers);
            highest = highest.max(*registers.values().max().unwrap_or(&0));
        }
        highest.into_some()
    }
}
