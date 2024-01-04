use std::collections::HashSet;

use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

pub struct Solution01;
impl Solution01 {
    fn parse(input: ProblemInput) -> Vec<(usize, i128)> {
        input
            .string()
            .split(", ")
            .map(|s| {
                let dir = s.starts_with('R') as usize;
                let length = s[1..].parse().unwrap();
                (dir, length)
            })
            .collect_vec()
    }
}

impl Solution for Solution01 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let instructions = Self::parse(input);
        let mut pos = Position { x: 0, y: 0 };
        let mut dir = Direction::North;

        for (turn, length) in instructions {
            dir = dir.turn()[turn];
            pos = pos.advance_by(dir, length);
        }

        (pos.x.abs() + pos.y.abs()).into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let instructions = Self::parse(input);
        let mut pos = Position { x: 0, y: 0 };
        let mut dir = Direction::North;

        let mut visited = HashSet::from([pos]);
        for (turn, length) in instructions {
            dir = dir.turn()[turn];
            for _ in 0..length {
                pos = pos.advance_by(dir, 1);
                if visited.contains(&pos) {
                    return (pos.x.abs() + pos.y.abs()).into_some();
                }
                visited.insert(pos);
            }
        }

        None
    }
}
