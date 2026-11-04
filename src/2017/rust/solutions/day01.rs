use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution01;
impl Solution01 {}

impl Solution for Solution01 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(3),
            ProblemResult::U32(1144),
            ProblemResult::U32(4),
            ProblemResult::U32(1194),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        input
            .string()
            .bytes()
            .circular_tuple_windows()
            .map(|(a, b)| if a == b { (a - b'0') as u32 } else { 0 })
            .sum::<u32>()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let list = input.string().bytes().collect_vec();
        assert!(list.len() & 1 == 0);
        list.iter()
            .zip(list.iter().cycle().skip(list.len() / 2))
            .map(|(a, b)| if a == b { (a - b'0') as u32 } else { 0 })
            .sum::<u32>()
            .to_result()
    }
}
