use std::collections::HashSet;

use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution09;
impl Solution09 {
    fn parse(input: ProblemInput) -> Vec<(Direction, i128)> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (dir, steps) = l.split_once(' ').unwrap();
                (dir.try_into().unwrap(), steps.parse().unwrap())
            })
            .collect()
    }

    fn move_rope(motions: &[(Direction, i128)], rope_size: usize) -> usize {
        let mut rope = vec![Position::zero(); rope_size];
        let mut visited = HashSet::from([rope[rope_size - 1]]);

        for &(dir, steps) in motions {
            for _ in 0..steps {
                rope[0] = rope[0].advance_by(dir, 1);
                for i in 1..rope_size {
                    // After one knot moved, two knots can only be apart by a moore neighborhood of 2. Easily list all cases.
                    let (delta_x, delta_y) = (rope[i - 1].x - rope[i].x, rope[i - 1].y - rope[i].y);
                    match (delta_x, delta_y) {
                        (-2, -2) => rope[i] = rope[i].advance_by(Direction::West, 1).advance_by(Direction::North, 1),
                        (-2, -1) => rope[i] = rope[i].advance_by(Direction::West, 1).advance_by(Direction::North, 1),
                        (-2, 0) => rope[i] = rope[i].advance_by(Direction::West, 1),
                        (-2, 1) => rope[i] = rope[i].advance_by(Direction::West, 1).advance_by(Direction::South, 1),
                        (-2, 2) => rope[i] = rope[i].advance_by(Direction::West, 1).advance_by(Direction::South, 1),

                        (-1, -2) => rope[i] = rope[i].advance_by(Direction::North, 1).advance_by(Direction::West, 1),
                        (-1, -1) => (),
                        (-1, 0) => (),
                        (-1, 1) => (),
                        (-1, 2) => rope[i] = rope[i].advance_by(Direction::South, 1).advance_by(Direction::West, 1),

                        (0, -2) => rope[i] = rope[i].advance_by(Direction::North, 1),
                        (0, -1) => (),
                        (0, 0) => (),
                        (0, 1) => (),
                        (0, 2) => rope[i] = rope[i].advance_by(Direction::South, 1),

                        (1, -2) => rope[i] = rope[i].advance_by(Direction::North, 1).advance_by(Direction::East, 1),
                        (1, -1) => (),
                        (1, 0) => (),
                        (1, 1) => (),
                        (1, 2) => rope[i] = rope[i].advance_by(Direction::South, 1).advance_by(Direction::East, 1),

                        (2, -2) => rope[i] = rope[i].advance_by(Direction::East, 1).advance_by(Direction::North, 1),
                        (2, -1) => rope[i] = rope[i].advance_by(Direction::East, 1).advance_by(Direction::North, 1),
                        (2, 0) => rope[i] = rope[i].advance_by(Direction::East, 1),
                        (2, 1) => rope[i] = rope[i].advance_by(Direction::East, 1).advance_by(Direction::South, 1),
                        (2, 2) => rope[i] = rope[i].advance_by(Direction::East, 1).advance_by(Direction::South, 1),
                        _ => unreachable!(),
                    }
                }

                visited.insert(rope[rope_size - 1]);
            }
        }

        visited.len()
    }
}

impl Solution for Solution09 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(88),
            ProblemResult::USize(6314),
            ProblemResult::USize(36),
            ProblemResult::USize(2504),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let motions = Self::parse(input);
        let tail_visited = Self::move_rope(&motions, 2);
        tail_visited.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let motions = Self::parse(input);
        let tail_visited = Self::move_rope(&motions, 10);
        tail_visited.to_result()
    }
}
