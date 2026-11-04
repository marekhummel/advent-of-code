use aoc_lib::cartesian::{HexDirection, HexIndex};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution11;
impl Solution11 {
    fn parse(input: ProblemInput) -> Vec<HexDirection> {
        input.string().split(',').flat_map(|d| d.try_into()).collect_vec()
    }
}

impl Solution for Solution11 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U128(3),
            ProblemResult::U128(808),
            ProblemResult::U128(3),
            ProblemResult::U128(1556),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let path = Self::parse(input);

        let start = HexIndex { x: 0, y: 0 };
        let mut target = start.clone();
        for dir in path {
            target = target.step(&dir);
        }

        start.dist(&target).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let path = Self::parse(input);

        let start = HexIndex { x: 0, y: 0 };
        let mut target = start.clone();
        let mut max_dist = 0;
        for dir in path {
            target = target.step(&dir);
            max_dist = max_dist.max(start.dist(&target))
        }

        max_dist.to_result()
    }
}
