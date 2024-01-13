use std::usize;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::solution::Solution;
use aoc_lib::specific::knothash::KnotHash;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

pub struct Solution10;
impl Solution10 {}

impl Solution for Solution10 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let n = if is_sample { 5 } else { 256 };
        let lengths = input.string().split(',').parsed::<usize>().collect_vec();

        let mut knot = KnotHash::custom(n, lengths);
        knot.round();

        (knot.circle[0] as u32 * knot.circle[1] as u32).into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut knot = KnotHash::new(&input.string());

        let hash = knot.hash().into_iter().map(|d| format!("{d:02x}")).join("");
        hash.into_some()
    }
}
