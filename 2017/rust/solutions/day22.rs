use std::collections::HashMap;

use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

#[derive(Clone, PartialEq)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

pub struct Solution22;
impl Solution22 {
    fn parse(input: ProblemInput) -> HashMap<Position, State> {
        let grid = input.grid();
        let center = Position::new(grid.size.width as i128 / 2, grid.size.height as i128 / 2);

        grid.enumerate()
            .filter(|(_, c)| **c == '#')
            .map(|(idx, _)| {
                (
                    Position::new(idx.i as i128 - center.x, idx.j as i128 - center.y),
                    State::Infected,
                )
            })
            .collect()
    }

    fn spread<F>(grid: &mut HashMap<Position, State>, burst_func: F, iterations: u32) -> u32
    where
        F: Fn(&State, &Direction) -> (Direction, State),
    {
        let mut pos = Position::zero();
        let mut dir = Direction::North;
        let mut infections = 0;
        for _ in 0..iterations {
            let updated_state = Self::burst(grid, &mut pos, &mut dir, &burst_func);

            if updated_state == State::Infected {
                infections += 1;
            }
        }

        infections
    }

    fn burst<F>(grid: &mut HashMap<Position, State>, pos: &mut Position, dir: &mut Direction, burst_func: F) -> State
    where
        F: Fn(&State, &Direction) -> (Direction, State),
    {
        let state = grid.entry(*pos).or_insert(State::Clean);
        (*dir, *state) = burst_func(state, dir);

        *pos = pos.advance_by(*dir, 1);

        state.clone()
    }
}

impl Solution for Solution22 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(5587),
            ProblemResult::U32(5575),
            ProblemResult::U32(2511944),
            ProblemResult::U32(2511991),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut grid = Self::parse(input);

        let burst_func = |state: &State, dir: &Direction| match state {
            State::Clean => (dir.left(), State::Infected),
            State::Infected => (dir.right(), State::Clean),
            _ => unreachable!(),
        };

        let infections = Self::spread(&mut grid, burst_func, 10_000);
        infections.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut grid = Self::parse(input);

        let burst_func = |state: &State, dir: &Direction| match state {
            State::Clean => (dir.left(), State::Weakened),
            State::Weakened => (*dir, State::Infected),
            State::Infected => (dir.right(), State::Flagged),
            State::Flagged => (dir.inverse(), State::Clean),
        };

        let infections = Self::spread(&mut grid, burst_func, 10_000_000);
        infections.to_result()
    }
}
