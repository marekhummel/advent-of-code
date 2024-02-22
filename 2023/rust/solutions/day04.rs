use aoc_lib::iterator::ParsedExt;
use itertools::Itertools;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use std::collections::HashSet;

type Hand = HashSet<u8>;

pub struct Solution04;

impl Solution04 {
    fn parse(&self, data: ProblemInput) -> Vec<(Hand, Hand)> {
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

    fn compute_value(&self, winning: &Hand, mine: &Hand) -> u32 {
        match self.count_hits(winning, mine) {
            0 => 0,
            h => 1 << (h - 1),
        }
    }

    fn count_hits(&self, winning: &Hand, mine: &Hand) -> usize {
        winning.intersection(mine).count()
    }
}

impl Solution for Solution04 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        self.parse(input)
            .iter()
            .map(|(winning, mine)| self.compute_value(winning, mine))
            .sum::<u32>()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut copies = vec![1u32; input.lines().len()];
        for (i, (winning, mine)) in self.parse(input).iter().enumerate() {
            let value = self.count_hits(winning, mine);
            let current = *copies.get(i).unwrap();
            for j in (i + 1)..=(i + value) {
                *copies.get_mut(j).unwrap() += current;
            }
        }

        copies.iter().sum::<u32>().to_result()
    }
}
