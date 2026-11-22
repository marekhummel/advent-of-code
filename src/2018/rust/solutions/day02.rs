use std::collections::HashSet;

use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
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

    (twos * threes).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let boxes = input
        .lines()
        .into_iter()
        .map(|boxid| boxid.char_indices().collect::<HashSet<_>>())
        .collect_vec();

    for [a, b] in boxes.into_iter().array_combinations() {
        if a.symmetric_difference(&b).count() == 2 {
            return a
                .intersection(&b)
                .sorted_by_key(|(idx, _)| idx)
                .map(|(_, c)| *c)
                .collect::<String>()
                .into();
        }
    }

    unreachable!()
}
