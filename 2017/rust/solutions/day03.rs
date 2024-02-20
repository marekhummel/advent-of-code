use std::collections::HashMap;

use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

struct SpiralPosIterator {
    pos: Position,
    dir: Direction,
    step: i64,
    steps: i64,
}

impl SpiralPosIterator {
    fn new() -> Self {
        // Non-trivial values, but results in first element being the 1
        SpiralPosIterator {
            pos: Position { x: 0, y: -1 },
            dir: Direction::South,
            step: -1,
            steps: 0,
        }
    }
}

impl Iterator for SpiralPosIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos = self.pos.advance_by(self.dir, 1);
        self.step += 1;

        if self.dir == Direction::East && self.step == self.steps + 1 {
            self.dir = self.dir.left();
            self.step = 1;
            self.steps += 2;
        } else if self.dir != Direction::East && self.step == self.steps {
            self.dir = self.dir.left();
            self.step = 0;
        }

        Some(self.pos)
    }
}

pub struct Solution03;
impl Solution03 {}

impl Solution for Solution03 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let n = input.string().parse::<usize>().unwrap();

        let mut it = SpiralPosIterator::new();
        let pos = it.nth(n - 1).unwrap();
        pos.dist(&Position::zero()).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let n = input.string().parse::<u64>().unwrap();
        let it = SpiralPosIterator::new();

        let mut spiral = HashMap::from([(Position::zero(), 1)]);
        for pos in it.skip(1) {
            let adjacent_sum: u64 = pos.moore_neighbors().into_iter().flat_map(|n| spiral.get(&n)).sum();

            if adjacent_sum > n {
                return adjacent_sum.to_result();
            }

            spiral.insert(pos, adjacent_sum);
        }

        unreachable!()
    }
}

//
