use aoc_lib::cartesian::{Direction, Index, Size};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution02;
impl Solution02 {
    fn parse(input: ProblemInput) -> Vec<Vec<Direction>> {
        input
            .lines()
            .into_iter()
            .map(|l| l.chars().map(|c| c.try_into().unwrap()).collect_vec())
            .collect_vec()
    }
}

impl Solution for Solution02 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let pad_size = Size::square(3);
        let mut pos = Index::new(1, 1);
        let mut code = 0;

        for instruction in Self::parse(input) {
            for dir in instruction {
                pos = pos.advance_check(dir, pad_size).unwrap_or(pos);
            }
            code = code * 10 + (pos.j * 3 + pos.i + 1)
        }

        code.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let pad_size = Size::square(5);
        let mut pos = Index::new(0, 2);
        let mut code = String::new();
        let valid_indices = (0usize..5)
            .flat_map(|j| {
                let s = j.abs_diff(2);
                (s..5 - s).map(move |i| Index { i, j })
            })
            .collect_vec();

        for instruction in Self::parse(input) {
            for dir in instruction {
                let new_pos = pos.advance_check(dir, pad_size).unwrap_or(pos);
                pos = if valid_indices.contains(&new_pos) { new_pos } else { pos };
            }
            code.push_str(format!("{:X}", valid_indices.iter().position(|i| *i == pos).unwrap() + 1).as_str())
        }

        code.to_result()
    }
}
