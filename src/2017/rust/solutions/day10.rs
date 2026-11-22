use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use aoc_lib::specific::knothash::KnotHash;
use itertools::Itertools;

pub fn part1(input: ProblemInput, is_sample: bool) -> ProblemResult {
    let n = if is_sample { 5 } else { 256 };
    let lengths = input.string().split(',').parsed::<usize>().collect_vec();

    let mut knot = KnotHash::custom(n, lengths);
    knot.round();

    (knot.circle[0] as u32 * knot.circle[1] as u32).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut knot = KnotHash::new(&input.string());

    let hash = knot.hash().into_iter().map(|d| format!("{d:02x}")).join("");
    hash.into()
}
