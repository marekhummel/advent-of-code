use aoc_lib::iterator::ParsedExt;
use itertools::Itertools;

use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use std::collections::HashSet;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    parse(input)
        .iter()
        .map(|(winning, mine)| compute_value(winning, mine))
        .sum::<u32>()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut copies = vec![1u32; input.lines().len()];
    for (i, (winning, mine)) in parse(input).iter().enumerate() {
        let value = count_hits(winning, mine);
        let current = *copies.get(i).unwrap();
        for j in (i + 1)..=(i + value) {
            *copies.get_mut(j).unwrap() += current;
        }
    }

    copies.iter().sum::<u32>().into()
}

type Hand = HashSet<u8>;

fn parse(data: ProblemInput) -> Vec<(Hand, Hand)> {
    let mut cards = vec![];
    for line in data.lines().iter() {
        let hand_strs = line.split(':').nth(1).unwrap().split('|');
        let parsed = hand_strs
            .map(|s| s.split_whitespace().parsed().collect::<HashSet<_>>())
            .take(2)
            .collect_tuple()
            .unwrap();
        cards.push(parsed)
    }

    cards
}

fn compute_value(winning: &Hand, mine: &Hand) -> u32 {
    match count_hits(winning, mine) {
        0 => 0,
        h => 1 << (h - 1),
    }
}

fn count_hits(winning: &Hand, mine: &Hand) -> usize {
    winning.intersection(mine).count()
}
