use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Input {
    Wire(String),
    Value(u16),
}

impl Input {
    fn value(&self, signals: &HashMap<String, u16>) -> u16 {
        match self {
            Input::Wire(x) => signals[x],
            Input::Value(val) => *val,
        }
    }

    fn ready(&self, signals: &HashMap<String, u16>) -> bool {
        match self {
            Input::Wire(x) => signals.contains_key(x),
            Input::Value(_) => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Signal(Input, String),
    And(Input, Input, String),
    Or(Input, Input, String),
    Not(Input, String),
    LShift(Input, Input, String),
    RShift(Input, Input, String),
}

pub struct Solution07;
impl Solution07 {
    fn parse(input: ProblemInput) -> Vec<Instruction> {
        input
            .lines()
            .into_iter()
            .map(|line| {
                let (connection, output) = line.split_once(" -> ").unwrap();
                let out = output.to_string();
                match &connection.split_whitespace().collect_vec()[..] {
                    [input] => Instruction::Signal(Self::parse_wire_input(input), out),
                    ["NOT", input] => Instruction::Not(Self::parse_wire_input(input), out),
                    [in1, "AND", in2] => {
                        Instruction::And(Self::parse_wire_input(in1), Self::parse_wire_input(in2), out)
                    }
                    [in1, "OR", in2] => Instruction::Or(Self::parse_wire_input(in1), Self::parse_wire_input(in2), out),
                    [in1, "LSHIFT", in2] => {
                        Instruction::LShift(Self::parse_wire_input(in1), Self::parse_wire_input(in2), out)
                    }
                    [in1, "RSHIFT", in2] => {
                        Instruction::RShift(Self::parse_wire_input(in1), Self::parse_wire_input(in2), out)
                    }
                    _ => unreachable!(),
                }
            })
            .collect_vec()
    }

    fn parse_wire_input(wire_input: &str) -> Input {
        match wire_input.parse::<u16>() {
            Ok(val) => Input::Value(val),
            Err(_) => Input::Wire(wire_input.to_string()),
        }
    }

    fn build_circuit(instructions: &[Instruction]) -> HashMap<String, u16> {
        let mut instruction_deque: VecDeque<_> = instructions.iter().collect();
        let mut signals = HashMap::new();
        while let Some(inst) = instruction_deque.pop_front() {
            let (wire, value) = match inst {
                Instruction::Signal(in1, output) if in1.ready(&signals) => (output, in1.value(&signals)),
                Instruction::Not(in1, output) if in1.ready(&signals) => (output, !in1.value(&signals)),
                Instruction::And(in1, in2, output) if in1.ready(&signals) && in2.ready(&signals) => {
                    (output, in1.value(&signals) & in2.value(&signals))
                }
                Instruction::Or(in1, in2, output) if in1.ready(&signals) && in2.ready(&signals) => {
                    (output, in1.value(&signals) | in2.value(&signals))
                }
                Instruction::LShift(in1, in2, output) if in1.ready(&signals) && in2.ready(&signals) => {
                    (output, in1.value(&signals) << in2.value(&signals))
                }
                Instruction::RShift(in1, in2, output) if in1.ready(&signals) && in2.ready(&signals) => {
                    (output, in1.value(&signals) >> in2.value(&signals))
                }
                _ => {
                    instruction_deque.push_back(inst);
                    continue;
                }
            };

            // Make sure not to overwrite value in part 2
            if let Entry::Vacant(ve) = signals.entry(wire.clone()) {
                ve.insert(value);
            }
        }

        signals
    }
}

impl Solution for Solution07 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let target = if is_sample { "i" } else { "a" };
        let signals = Self::build_circuit(&Self::parse(input));
        signals[target].to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let mut instructions = Self::parse(input);
        let signals1 = Self::build_circuit(&instructions);

        let target = if is_sample { "i" } else { "a" };
        let rewire_instruction = Instruction::Signal(Input::Value(signals1[target]), "b".to_string());
        instructions.insert(0, rewire_instruction);
        let signals2 = Self::build_circuit(&instructions);
        signals2[target].to_result()
    }
}
