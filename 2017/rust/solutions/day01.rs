use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

pub struct Solution01;
impl Solution01 {}

impl Solution for Solution01 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        input
            .string()
            .bytes()
            .circular_tuple_windows()
            .map(|(a, b)| if a == b { (a - b'0') as u32 } else { 0 })
            .sum::<u32>()
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let list = input.string().bytes().collect_vec();
        assert!(list.len() & 1 == 0);
        list.iter()
            .zip(list.iter().cycle().skip(list.len() / 2))
            .map(|(a, b)| if a == b { (a - b'0') as u32 } else { 0 })
            .sum::<u32>()
            .into_some()
    }
}
