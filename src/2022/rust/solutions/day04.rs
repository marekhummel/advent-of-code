use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

type Pair = ((u32, u32), (u32, u32));

pub struct Solution04;
impl Solution04 {
    fn parse(input: ProblemInput) -> Vec<Pair> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (a0, b0, a1, b1) = l.split(&['-', ',']).parsed().collect_tuple().unwrap();
                ((a0, b0), (a1, b1))
            })
            .collect()
    }

    fn fully_contain(((a0, b0), (a1, b1)): &Pair) -> bool {
        (a0 <= a1 && b1 <= b0) || (a1 <= a0 && b0 <= b1)
    }

    fn overlap(((a0, b0), (a1, b1)): &Pair) -> bool {
        a0 <= b1 && a1 <= b0
    }
}

impl Solution for Solution04 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(2),
            ProblemResult::USize(538),
            ProblemResult::USize(4),
            ProblemResult::USize(792),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let pairs = Self::parse(input);
        pairs.into_iter().filter(Self::fully_contain).count().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let pairs = Self::parse(input);
        pairs.into_iter().filter(Self::overlap).count().to_result()
    }
}
