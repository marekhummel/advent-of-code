use aoc_lib::graph::{self, Graph};
use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution12;
impl Solution12 {
    fn parse(input: ProblemInput) -> Graph<u32> {
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
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let pipes = Self::parse(input);
        let components = graph::components(&pipes);

        let group0 = components.into_iter().find(|c| c.contains(&0)).unwrap();
        group0.len().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let pipes = Self::parse(input);
        let components = graph::components(&pipes);
        components.len().to_result()
    }
}
