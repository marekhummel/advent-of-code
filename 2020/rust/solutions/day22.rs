use std::collections::HashSet;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

type Hand = Vec<u8>;

pub struct Solution22;
impl Solution22 {
    fn parse(input: ProblemInput) -> (Hand, Hand) {
        let lines = input.lines();
        let players = lines.split(|l| l.is_empty());
        let cards = players.map(|hand| hand.iter().skip(1).parsed().collect());
        cards.collect_tuple().unwrap()
    }

    fn combat(mut player1: Hand, mut player2: Hand) -> u32 {
        while !player1.is_empty() && !player2.is_empty() {
            let (top1, top2) = Self::draw_cards(&mut player1, &mut player2);
            Self::update_winner(&mut player1, &mut player2, top1, top2, top1 > top2);
        }

        Self::eval_winning_score(&player1, &player2)
    }

    fn recursive_combat(mut player1: Hand, mut player2: Hand) -> (bool, u32) {
        let mut seen = HashSet::new();

        while !player1.is_empty() && !player2.is_empty() {
            // Infinite games
            if seen.contains(&(player1.clone(), player2.clone())) {
                return (true, Self::eval_winning_score(&player1, &player2));
            }
            seen.insert((player1.clone(), player2.clone()));

            // Round
            let (top1, top2) = Self::draw_cards(&mut player1, &mut player2);
            let winner = if top1 as usize <= player1.len() && top2 as usize <= player2.len() {
                // Recursive game
                let sub_deck1 = player1[0..top1 as usize].to_vec();
                let sub_deck2 = player2[0..top2 as usize].to_vec();
                let (player1_won, _) = Self::recursive_combat(sub_deck1, sub_deck2);
                player1_won
            } else {
                // Normal round
                top1 > top2
            };

            Self::update_winner(&mut player1, &mut player2, top1, top2, winner);
        }

        (player2.is_empty(), Self::eval_winning_score(&player1, &player2))
    }

    fn draw_cards(player1: &mut Hand, player2: &mut Hand) -> (u8, u8) {
        player1.rotate_left(1);
        player2.rotate_left(1);
        let top1 = player1.pop().unwrap();
        let top2 = player2.pop().unwrap();
        (top1, top2)
    }

    fn update_winner(player1: &mut Hand, player2: &mut Hand, top1: u8, top2: u8, player1_won: bool) {
        if player1_won {
            player1.extend([top1, top2]);
        } else {
            player2.extend([top2, top1]);
        }
    }

    fn eval_winning_score(player1: &Hand, player2: &Hand) -> u32 {
        let winner = if player1.is_empty() { player2 } else { player1 };
        winner.iter().rev().zip(1..).map(|(c, m)| *c as u32 * m).sum()
    }
}

impl Solution for Solution22 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(306),
            ProblemResult::U32(34255),
            ProblemResult::U32(291),
            ProblemResult::U32(33368),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (player1, player2) = Self::parse(input);
        let score = Self::combat(player1, player2);
        score.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (player1, player2) = Self::parse(input);
        let (_, score) = Self::recursive_combat(player1, player2);
        score.to_result()
    }
}
