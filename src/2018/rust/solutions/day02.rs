use std::collections::HashSet;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution02;
impl Solution02 {}

impl Solution for Solution02 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (mut twos, mut threes) = (0u32, 0u32);

        for boxid in input.lines() {
            let letters = boxid.chars().counts();
            if letters.iter().any(|(_, count)| *count == 2) {
                twos += 1;
            }
            if letters.iter().any(|(_, count)| *count == 3) {
                threes += 1;
            }
        }

        (twos * threes).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let boxes = input
            .lines()
            .into_iter()
            .map(|boxid| boxid.char_indices().collect::<HashSet<_>>())
            .collect_vec();

        for (a, b) in boxes.into_iter().tuple_combinations() {
            if a.symmetric_difference(&b).count() == 2 {
                return a
                    .intersection(&b)
                    .sorted_by_key(|(idx, _)| idx)
                    .map(|(_, c)| *c)
                    .collect::<String>()
                    .to_result();
            }
        }

        unreachable!()
    }
}
