use std::collections::{HashMap, HashSet};

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut memory = parse(input);
    let mut seen = HashSet::new();
    for i in 0.. {
        if seen.contains(&memory) {
            return i.into();
        }

        seen.insert(memory.clone());
        redistribute(&mut memory);
    }

    unreachable!()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut memory = parse(input);
    let mut seen = HashMap::new();
    for i in 0u32.. {
        if let Some(pi) = seen.get(&memory) {
            let loop_sz: u32 = i - *pi;
            return loop_sz.into();
        }

        seen.insert(memory.clone(), i);
        redistribute(&mut memory);
    }

    unreachable!()
}

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
