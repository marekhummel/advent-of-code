use std::collections::VecDeque;

use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution09;
impl Solution09 {
    fn parse(input: ProblemInput) -> (usize, usize) {
        let game = input.string();
        let words = game.split_whitespace().collect_vec();
        (words[0].parse().unwrap(), words[6].parse().unwrap())
    }

    // Use VecDeque over Vec to make use of links, and rotate instead of insert to avoid O(n)
    fn play(num_players: usize, last_marble: usize) -> usize {
        let mut scores = vec![0; num_players];
        let mut circle = VecDeque::from([0]);
        for marble in 1..=last_marble {
            if marble % 23 == 0 {
                let player = (marble - 1) % num_players;
                circle.rotate_right(7);
                let removed_marble = circle.pop_back().unwrap();
                scores[player] += marble + removed_marble;
                circle.rotate_left(1);
            } else {
                circle.rotate_left(1);
                circle.push_back(marble);
            }
        }

        scores.into_iter().max().unwrap()
    }
}

impl Solution for Solution09 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (num_players, last_marble) = Self::parse(input);

        let high_score = Self::play(num_players, last_marble);
        high_score.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (num_players, last_marble) = Self::parse(input);

        let high_score = Self::play(num_players, last_marble * 100);
        high_score.to_result()
    }
}
