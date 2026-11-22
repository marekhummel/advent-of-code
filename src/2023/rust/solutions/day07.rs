use std::collections::HashMap;

use itertools::Itertools;

use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    solve(input, false).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    solve(input, true).into()
}

#[derive(Debug)]
struct CamelHand {
    hand: String,
    bet: u32,
}

fn solve(input: ProblemInput, with_joker: bool) -> u32 {
    let mut hands = parse(input);
    hands.sort_by_key(|h| {
        (
            eval_hand(&h.hand, with_joker),
            eval_cards_single(&h.hand, with_joker),
        )
    });

    // for h in &hands {
    //     println!(
    //         "{h:?}: {0}, {1:?}",
    //         eval_hand(&h.hand),
    //         eval_cards_single2(&h.hand)
    //     )
    // }

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) as u32 * hand.bet)
        .sum()
}

fn parse(input: ProblemInput) -> Vec<CamelHand> {
    input
        .lines()
        .iter()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(hd, bet)| CamelHand {
            hand: String::from(hd),
            bet: bet.parse().unwrap(),
        })
        .collect_vec()
}

fn eval_hand(hand: &str, with_joker: bool) -> u8 {
    let groups = hand
        .chars()
        .filter(|c| !with_joker || c != &'J')
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0u8) += 1;
            map
        });

    match groups.values().sorted().as_slice() {
        // five cards
        [5] => 6,             // five of a kind
        [1, 4] => 5,          // four of a kind
        [2, 3] => 4,          // full house
        [1, 1, 3] => 3,       // three of a kind
        [1, 2, 2] => 2,       // two pair
        [1, 1, 1, 2] => 1,    // one pair
        [1, 1, 1, 1, 1] => 0, // high card
        // one joker
        [4] => 6,
        [1, 3] => 5,
        [2, 2] => 4,
        [1, 1, 2] => 3,
        [1, 1, 1, 1] => 1,
        // two jokers
        [3] => 6,
        [1, 2] => 5,
        [1, 1, 1] => 3,
        // three jokers
        [2] => 6,
        [1, 1] => 5,
        // four jokers
        [1] => 6,
        // five jokers
        [] => 6,
        _ => unreachable!(),
    }
}

fn eval_cards_single(hand: &str, with_joker: bool) -> (u8, u8, u8, u8, u8) {
    let closure = match with_joker {
        false => |c: char| c.to_digit(10).unwrap_or_else(|| "TJQKA".find(c).unwrap() as u32 + 10) as u8,
        true => |c: char| {
            c.to_digit(10)
                .unwrap_or_else(|| "TQKA".find(c).map_or(1, |i| i as u32 + 10)) as u8
        },
    };

    hand.chars().map(closure).take(5).collect_tuple().unwrap()
}
