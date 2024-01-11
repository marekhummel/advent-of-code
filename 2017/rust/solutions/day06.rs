use std::collections::{HashMap, HashSet};

use aoc_lib::iterator::ParsedExt;
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution06;
impl Solution06 {
    fn parse(input: ProblemInput) -> Vec<u32> {
        input.string().split_whitespace().parsed().collect()
    }

    fn redistribute(memory: &mut [u32]) {
        let max_blocks = *memory.iter().max().unwrap();
        let chosen = memory.iter().position(|b| *b == max_blocks).unwrap();
        memory[chosen] = 0;
        for i in 0..max_blocks as usize {
            let bi = (chosen + 1 + i) % memory.len();
            memory[bi] += 1;
        }
    }
}

impl Solution for Solution06 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut memory = Self::parse(input);
        let mut seen = HashSet::new();
        for i in 0.. {
            if seen.contains(&memory) {
                return i.into_some();
            }

            seen.insert(memory.clone());
            Self::redistribute(&mut memory);
        }

        unreachable!()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut memory = Self::parse(input);
        let mut seen = HashMap::new();
        for i in 0u32.. {
            if let Some(pi) = seen.get(&memory) {
                let loop_sz: u32 = i - *pi;
                return loop_sz.into_some();
            }

            seen.insert(memory.clone(), i);
            Self::redistribute(&mut memory);
        }

        unreachable!()
    }
}
