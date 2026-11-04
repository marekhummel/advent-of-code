use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution01;
impl Solution01 {}

impl Solution for Solution01 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(7),
            ProblemResult::USize(1390),
            ProblemResult::USize(5),
            ProblemResult::USize(1457),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let depths = input.lines().into_iter().parsed::<u16>();
        let increasing = depths.tuple_windows().filter(|(a, b)| a < b).count();

        increasing.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let depths = input.lines().into_iter().parsed::<u16>();
        let windows = depths.tuple_windows().map(|(a, b, c)| a + b + c);
        let increasing = windows.tuple_windows().filter(|(a, b)| a < b).count();

        increasing.to_result()
    }
}
