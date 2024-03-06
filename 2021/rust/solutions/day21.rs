use std::collections::HashMap;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::{iproduct, Itertools};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    pos: u32,
    score: u32,
}

impl Player {
    fn new(pos: u32) -> Self {
        Self { pos, score: 0 }
    }

    fn advance(&mut self, roll: u32) {
        self.pos = (self.pos - 1 + roll) % 10 + 1;
        self.score += self.pos;
    }
}

struct DeterministicDie {
    value: u32,
    total_rolls: u32,
}

impl DeterministicDie {
    fn new() -> Self {
        Self {
            value: 0,
            total_rolls: 0,
        }
    }
}

impl Iterator for DeterministicDie {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = self.value % 100 + 1;
        self.total_rolls += 1;
        Some(self.value)
    }
}

pub struct Solution21;
impl Solution21 {
    fn parse(input: ProblemInput) -> (u32, u32) {
        input
            .lines()
            .into_iter()
            .map(|l| l.split_ascii_whitespace().last().unwrap().to_string())
            .parsed()
            .collect_tuple()
            .unwrap()
    }

    fn dirac_dice(turn: usize, players: [Player; 2], cache: &mut HashMap<(usize, [Player; 2]), [u64; 2]>) -> [u64; 2] {
        // We had this setting before
        if let Some(&wins) = cache.get(&(turn, players)) {
            return wins;
        }

        // Run thorugh each of the possible die rolls
        let mut wins = [0, 0];
        for (r1, r2, r3) in iproduct!([1, 2, 3], [1, 2, 3], [1, 2, 3]) {
            let mut new_players = players;

            new_players[turn].advance(r1 + r2 + r3);
            // Player won, just increase score by 1
            if new_players[turn].score >= 21 {
                wins[turn] += 1;
            } else {
                // No winner yet, recurse with new setting
                let subwins = Self::dirac_dice(1 - turn, new_players, cache);
                wins[0] += subwins[0];
                wins[1] += subwins[1];
            }
        }

        cache.insert((turn, players), wins);
        wins
    }
}

impl Solution for Solution21 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(739785),
            ProblemResult::U32(888735),
            ProblemResult::U64(444356092776315),
            ProblemResult::U64(647608359455719),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (pos1, pos2) = Self::parse(input);
        let mut active = Player::new(pos1);
        let mut other = Player::new(pos2);
        let mut die = DeterministicDie::new();

        while active.score < 1000 && other.score < 1000 {
            let rolls = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();
            active.advance(rolls);
            (active, other) = (other, active);
        }

        (active.score * die.total_rolls).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (pos1, pos2) = Self::parse(input);
        let players = [Player::new(pos1), Player::new(pos2)];
        let wins = Self::dirac_dice(0, players, &mut HashMap::new());
        (wins[0].max(wins[1])).to_result()
    }
}
