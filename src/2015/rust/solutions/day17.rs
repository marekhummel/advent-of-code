use std::collections::HashMap;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, is_sample: bool) -> ProblemResult {
    let eggnog = if is_sample { 25 } else { 150 };
    let containers = parse(input);
    fill(&containers, eggnog, 0).values().sum::<u32>().into()
}

pub fn part2(input: ProblemInput, is_sample: bool) -> ProblemResult {
    let eggnog = if is_sample { 25 } else { 150 };
    let containers = parse(input);

    fill(&containers, eggnog, 0)
        .into_iter()
        .min_by_key(|(containers, _)| *containers)
        .unwrap()
        .1
        .into()
}

fn parse(input: ProblemInput) -> Vec<u8> {
    input.lines().into_iter().parsed().collect_vec()
}

fn fill(containers: &[u8], eggnog: i16, used: u8) -> HashMap<u8, u32> {
    if containers.is_empty() {
        return if eggnog == 0 {
            HashMap::from([(used, 1)])
        } else {
            HashMap::new()
        };
    }

    let mut results = HashMap::new();
    fill(&containers[1..], eggnog, used)
        .into_iter()
        .for_each(|(u, c)| *results.entry(u).or_insert(0) += c);
    fill(&containers[1..], eggnog - containers[0] as i16, used + 1)
        .into_iter()
        .for_each(|(u, c)| *results.entry(u).or_insert(0) += c);
    results
}
