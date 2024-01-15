use std::collections::HashMap;

use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

#[derive(Debug)]
struct Turing {
    tape: HashMap<i32, bool>,
    state: u8,
    cursor: i32,
    diagnostic_after: usize,
    rules: HashMap<(u8, bool), (bool, i32, u8)>,
}

impl Turing {
    fn parse(input: ProblemInput) -> Self {
        let lines = input.lines();
        let blocks = lines.split(|l| l.is_empty()).collect_vec();

        let start_state = blocks[0][0].chars().nth_back(1).unwrap() as u8 - b'A';
        let diagnostic = blocks[0][1].split_whitespace().nth_back(1).unwrap().parse().unwrap();

        let mut rules = HashMap::new();
        for block in &blocks[1..] {
            let state = block[0].chars().nth_back(1).unwrap() as u8 - b'A';

            for transition in block[1..].chunks_exact(4) {
                let curr_value = transition[0].chars().nth_back(1).unwrap() == '1';
                let new_value = transition[1].chars().nth_back(1).unwrap() == '1';
                let left = transition[2].split_whitespace().last().unwrap() == "left.";
                let new_state = transition[3].chars().nth_back(1).unwrap() as u8 - b'A';

                rules.insert((state, curr_value), (new_value, if left { -1 } else { 1 }, new_state));
            }
        }

        Turing {
            tape: HashMap::new(),
            state: start_state,
            cursor: 0,
            diagnostic_after: diagnostic,
            rules,
        }
    }

    fn step(&mut self) {
        let tape_entry = self.tape.entry(self.cursor).or_insert(false);

        let (new_value, dir, new_state) = self.rules[&(self.state, *tape_entry)];

        *tape_entry = new_value;
        self.cursor += dir;
        self.state = new_state;
    }

    fn checksum(&self) -> usize {
        self.tape.values().filter(|v| **v).count()
    }
}

pub struct Solution25;
impl Solution25 {}

impl Solution for Solution25 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut turing = Turing::parse(input);

        for _ in 0..turing.diagnostic_after {
            turing.step();
        }

        turing.checksum().into_some()
    }

    fn solve_version02(&self, _input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        // No part 2
        None
    }
}
