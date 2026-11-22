use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let depths = input.lines().into_iter().parsed::<u16>();
    let increasing = depths.tuple_windows().filter(|(a, b)| a < b).count();

    increasing.into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let depths = input.lines().into_iter().parsed::<u16>();
    let windows = depths.tuple_windows().map(|(a, b, c)| a + b + c);
    let increasing = windows.tuple_windows().filter(|(a, b)| a < b).count();

    increasing.into()
}
