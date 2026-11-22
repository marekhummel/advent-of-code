use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input
        .string()
        .bytes()
        .circular_tuple_windows()
        .map(|(a, b)| if a == b { (a - b'0') as u32 } else { 0 })
        .sum::<u32>()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let list = input.string().bytes().collect_vec();
    assert!(list.len() & 1 == 0);
    list.iter()
        .zip(list.iter().cycle().skip(list.len() / 2))
        .map(|(a, b)| if a == b { (a - b'0') as u32 } else { 0 })
        .sum::<u32>()
        .into()
}
