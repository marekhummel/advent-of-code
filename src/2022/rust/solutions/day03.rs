use std::collections::HashSet;

use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let rucksacks = input.lines().into_iter().map(|l| {
        let mut first = l.chars().collect_vec();
        let second = first.drain(first.len() / 2..).collect_vec();
        (
            first.into_iter().collect::<HashSet<_>>(),
            second.into_iter().collect::<HashSet<_>>(),
        )
    });

    let failed = rucksacks.map(|(first, second)| *first.intersection(&second).next().unwrap());
    let priorities = failed.map(|item| priority(item) as u32);
    priorities.sum::<u32>().into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let rucksacks = input.lines().into_iter().map(|l| l.chars().collect::<HashSet<_>>());

    let badges = rucksacks
        .tuples()
        .map(|(a, b, c)| &(&a & &b) & &c)
        .map(|badge_set| badge_set.into_iter().next().unwrap());

    let priorities = badges.map(|item| priority(item) as u32);
    priorities.sum::<u32>().into()
}

fn priority(item: char) -> u8 {
    if item.is_ascii_lowercase() {
        item as u8 - b'a' + 1
    } else {
        item as u8 - b'A' + 27
    }
}
