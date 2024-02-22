use std::collections::HashMap;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution17;
impl Solution17 {
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
        Self::fill(&containers[1..], eggnog, used)
            .into_iter()
            .for_each(|(u, c)| *results.entry(u).or_insert(0) += c);
        Self::fill(&containers[1..], eggnog - containers[0] as i16, used + 1)
            .into_iter()
            .for_each(|(u, c)| *results.entry(u).or_insert(0) += c);
        results
    }
}

impl Solution for Solution17 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(4),
            ProblemResult::U32(1304),
            ProblemResult::U32(3),
            ProblemResult::U32(18),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let eggnog = if is_sample { 25 } else { 150 };
        let containers = Self::parse(input);
        Self::fill(&containers, eggnog, 0).values().sum::<u32>().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let eggnog = if is_sample { 25 } else { 150 };
        let containers = Self::parse(input);

        Self::fill(&containers, eggnog, 0)
            .into_iter()
            .min_by_key(|(containers, _)| *containers)
            .unwrap()
            .1
            .to_result()
    }
}
