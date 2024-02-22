use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::{Itertools, MinMaxResult};

pub struct Solution02;
impl Solution02 {
    fn parse(input: ProblemInput) -> Vec<Vec<u32>> {
        input
            .lines()
            .into_iter()
            .map(|l| l.split_whitespace().parsed().collect_vec())
            .collect_vec()
    }
}

impl Solution for Solution02 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let spreadsheet = Self::parse(input);

        spreadsheet
            .into_iter()
            .map(|row| {
                let MinMaxResult::MinMax(x, y) = row.iter().minmax() else {
                    return 0;
                };
                y - x
            })
            .sum::<u32>()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let spreadsheet = Self::parse(input);

        spreadsheet
            .into_iter()
            .map(|row| {
                let (a, b) = row
                    .into_iter()
                    .tuple_combinations()
                    .map(|(a, b)| (a.max(b), a.min(b)))
                    .find(|(a, b)| a % b == 0)
                    .unwrap();
                a / b
            })
            .sum::<u32>()
            .to_result()
    }
}
