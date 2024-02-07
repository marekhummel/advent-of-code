use std::cmp::Ordering;
use std::collections::HashMap;

use aoc_lib::cartesian::Position;
use aoc_lib::solution::Solution;
use aoc_lib::specific::intcode::Program;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution13;
impl Solution13 {}

impl Solution for Solution13 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        if is_sample {
            return None;
        }

        let mut screen = HashMap::new();
        let mut game = Program::init(&input.string());

        loop {
            let Some(x) = game.execute_until_output() else { break; };
            let Some(y) = game.execute_until_output() else { break; };
            let Some(tile) = game.execute_until_output() else { break; };

            assert!(!x.is_negative() && !y.is_negative());
            screen.insert(Position { x, y }, tile);
        }

        screen.values().filter(|tile| **tile == 2).count().into_some()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        if is_sample {
            return None;
        }

        let mut game = Program::init(&input.string());
        *game.memory.get_mut(&0).unwrap() = 2;

        let mut ball = 0;
        let mut paddle = 0;
        let mut score = 0;
        loop {
            // AI
            game.input.clear();
            game.input.push_back(match ball.cmp(&paddle) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            });

            // Get screen output
            let Some(x) = game.execute_until_output() else { break; };
            let Some(y) = game.execute_until_output() else { break; };

            if x == -1 && y == 0 {
                // Score update
                score = game.execute_until_output().unwrap();
            } else {
                // Tile update
                assert!(!x.is_negative() && !y.is_negative());
                match game.execute_until_output() {
                    Some(3) => paddle = x,
                    Some(4) => ball = x,
                    Some(_) => (),
                    None => break,
                }
            }
        }

        score.into_some()
    }
}
