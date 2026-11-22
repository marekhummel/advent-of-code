use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let depths = input.string().split(',').parsed::<u32>().collect_vec();
    let min_fuel_spent = min_fuel_spent(&depths, |td, d| td.abs_diff(d));
    min_fuel_spent.into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let depths = input.string().split(',').parsed::<u32>().collect_vec();
    let min_fuel_spent = min_fuel_spent(&depths, |td, d| td.abs_diff(d) * (td.abs_diff(d) + 1) / 2);
    min_fuel_spent.into()
}

fn min_fuel_spent<F: Fn(u32, u32) -> u32>(depths: &[u32], fuel_func: F) -> u32 {
    (0..*depths.iter().max().unwrap())
        .map(|td| depths.iter().map(|d| fuel_func(td, *d)).sum::<u32>())
        .min()
        .unwrap()
}
