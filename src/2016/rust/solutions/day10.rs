use std::collections::{HashMap, VecDeque};

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Take(usize, u32),
    Compare(usize, Target, Target),
}
#[derive(Debug, Clone, Copy)]
enum Target {
    Bot(usize),
    Output(usize),
}

type Exchange = (usize, u32, u32);

impl From<&[&str]> for Target {
    fn from(value: &[&str]) -> Self {
        assert_eq!(value.len(), 2);
        let id = value[1].parse().unwrap();
        match value[0] {
            "bot" => Target::Bot(id),
            "output" => Target::Output(id),
            _ => unreachable!(),
        }
    }
}

pub struct Solution10;
impl Solution10 {
    fn parse(input: ProblemInput) -> Vec<Instruction> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let words = l.split_whitespace().collect_vec();
                match words[0] {
                    "value" => {
                        let value = words[1].parse().unwrap();
                        Instruction::Take(words[5].parse().unwrap(), value)
                    }
                    "bot" => {
                        let bot = words[1].parse().unwrap();
                        let low = &words[5..7];
                        let high = &words[10..12];

                        Instruction::Compare(bot, low.into(), high.into())
                    }
                    _ => unreachable!(),
                }
            })
            .collect_vec()
    }

    fn run_instructions(instructions: Vec<Instruction>) -> (Vec<Exchange>, HashMap<usize, Vec<u32>>) {
        let mut bots = HashMap::new();
        let mut outputs = HashMap::new();

        let mut exchanges = VecDeque::new();
        for inst in instructions {
            if let Instruction::Take(bot, value) = inst {
                bots.entry(bot).or_insert(Vec::new()).push(value);
            } else {
                exchanges.push_back(inst)
            }
        }

        let mut completed_exchanges = Vec::new();
        while let Some(inst @ Instruction::Compare(bot, low_trg, high_trg)) = exchanges.pop_front() {
            if let Some((v1, v2)) = bots.get(&bot).and_then(|vals| vals.iter().cloned().collect_tuple()) {
                let (low, high) = (v1.min(v2), v1.max(v2));
                for (trg, value) in [(low_trg, low), (high_trg, high)] {
                    match trg {
                        Target::Bot(b) => bots.entry(b).or_insert(Vec::new()).push(value),
                        Target::Output(o) => outputs.entry(o).or_insert(Vec::new()).push(value),
                    }
                }

                completed_exchanges.push((bot, low, high));
            } else {
                exchanges.push_back(inst);
            }
        }

        (completed_exchanges, outputs)
    }
}

impl Solution for Solution10 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let instructions = Self::parse(input);
        let chips = if is_sample { (2, 5) } else { (17, 61) };

        let (exchanges, _) = Self::run_instructions(instructions);
        exchanges
            .into_iter()
            .find(|(_, low, high)| (*low, *high) == chips)
            .unwrap()
            .0
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let instructions = Self::parse(input);

        let (_, outputs) = Self::run_instructions(instructions);
        [0, 1, 2].iter().flat_map(|o| &outputs[o]).product::<u32>().to_result()
    }
}
