use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution06;
impl Solution06 {
    fn find_marker(datastream_str: String, length: usize) -> usize {
        let datastream = datastream_str.chars().collect_vec();

        for (i, window) in datastream.windows(length).enumerate() {
            if window.iter().unique().count() == length {
                return i + length;
            }
        }

        unreachable!()
    }
}

impl Solution for Solution06 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(7),
            ProblemResult::USize(1238),
            ProblemResult::USize(19),
            ProblemResult::USize(3037),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        Self::find_marker(input.string(), 4).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        Self::find_marker(input.string(), 14).to_result()
    }
}
