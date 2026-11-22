use std::collections::HashSet;

use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let motions = parse(input);
    let tail_visited = move_rope(&motions, 2);
    tail_visited.into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let motions = parse(input);
    let tail_visited = move_rope(&motions, 10);
    tail_visited.into()
}

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
