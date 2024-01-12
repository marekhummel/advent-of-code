use std::collections::{HashMap, HashSet};

use aoc_lib::graph;
use aoc_lib::iterator::ParsedExt;
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution12;
impl Solution12 {
    fn parse(input: ProblemInput) -> HashMap<u32, HashSet<u32>> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (src, trgs_str) = l.split_once("<->").unwrap();
                let trgs = trgs_str.split(',').parsed().collect();
                (src.trim().parse().unwrap(), trgs)
            })
            .collect()
    }
}

impl Solution for Solution12 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let pipes = Self::parse(input);
        let components = graph::components(&pipes);

        let group0 = components.into_iter().find(|c| c.contains(&0)).unwrap();
        group0.len().into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let pipes = Self::parse(input);
        let components = graph::components(&pipes);
        components.len().into_some()
    }
}
