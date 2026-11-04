use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution07;
impl Solution07 {
    fn min_fuel_spent<F: Fn(u32, u32) -> u32>(depths: &[u32], fuel_func: F) -> u32 {
        (0..*depths.iter().max().unwrap())
            .map(|td| depths.iter().map(|d| fuel_func(td, *d)).sum::<u32>())
            .min()
            .unwrap()
    }
}

impl Solution for Solution07 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(37),
            ProblemResult::U32(356179),
            ProblemResult::U32(168),
            ProblemResult::U32(99788435),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let depths = input.string().split(',').parsed::<u32>().collect_vec();
        let min_fuel_spent = Self::min_fuel_spent(&depths, |td, d| td.abs_diff(d));
        min_fuel_spent.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let depths = input.string().split(',').parsed::<u32>().collect_vec();
        let min_fuel_spent = Self::min_fuel_spent(&depths, |td, d| td.abs_diff(d) * (td.abs_diff(d) + 1) / 2);
        min_fuel_spent.to_result()
    }
}
