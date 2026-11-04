use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::knothash::KnotHash;
use itertools::Itertools;

pub struct Solution10;
impl Solution10 {}

impl Solution for Solution10 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(12),
            ProblemResult::U32(2928),
            ProblemResult::String("33efeb34ea91902bb2f59c9920caa6cd".to_string()),
            ProblemResult::String("0c2f794b2eb555f7830766bf8fb65a16".to_string()),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let n = if is_sample { 5 } else { 256 };
        let lengths = input.string().split(',').parsed::<usize>().collect_vec();

        let mut knot = KnotHash::custom(n, lengths);
        knot.round();

        (knot.circle[0] as u32 * knot.circle[1] as u32).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut knot = KnotHash::new(&input.string());

        let hash = knot.hash().into_iter().map(|d| format!("{d:02x}")).join("");
        hash.to_result()
    }
}
