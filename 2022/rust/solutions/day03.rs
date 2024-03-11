use std::collections::HashSet;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution03;
impl Solution03 {
    fn priority(item: char) -> u8 {
        if item.is_ascii_lowercase() {
            item as u8 - b'a' + 1
        } else {
            item as u8 - b'A' + 27
        }
    }
}

impl Solution for Solution03 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(157),
            ProblemResult::U32(8243),
            ProblemResult::U32(70),
            ProblemResult::U32(2631),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let rucksacks = input.lines().into_iter().map(|l| {
            let mut first = l.chars().collect_vec();
            let second = first.drain(first.len() / 2..).collect_vec();
            (
                first.into_iter().collect::<HashSet<_>>(),
                second.into_iter().collect::<HashSet<_>>(),
            )
        });

        let failed = rucksacks.map(|(first, second)| *first.intersection(&second).next().unwrap());
        let priorities = failed.map(|item| Self::priority(item) as u32);
        priorities.sum::<u32>().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let rucksacks = input.lines().into_iter().map(|l| l.chars().collect::<HashSet<_>>());

        let badges = rucksacks
            .tuples()
            .map(|(a, b, c)| &(&a & &b) & &c)
            .map(|badge_set| badge_set.into_iter().next().unwrap());

        let priorities = badges.map(|item| Self::priority(item) as u32);
        priorities.sum::<u32>().to_result()
    }
}
