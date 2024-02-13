use std::collections::HashSet;

use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
pub struct Solution03;

impl Solution03 {
    fn parse(input: ProblemInput) -> Vec<Direction> {
        input.string().chars().map(|c| c.try_into().unwrap()).collect_vec()
    }

    fn visit_houses(start: Position, directions: &[Direction]) -> HashSet<Position> {
        let mut houses: HashSet<Position> = directions
            .iter()
            .scan(start, |pos, dir| {
                *pos = pos.advance_by(*dir, 1);
                Some(*pos)
            })
            .collect();

        houses.insert(start);
        houses
    }
}

impl Solution for Solution03 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let directions = Self::parse(input);
        let start = Position::zero();
        let houses = Self::visit_houses(start, &directions);
        houses.len().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let directions = Self::parse(input);
        let robo_dirs = directions.iter().step_by(2).cloned().collect_vec();
        let santa_dirs = directions.iter().skip(1).step_by(2).cloned().collect_vec();

        let start = Position::zero();

        let mut houses = HashSet::new();
        houses.extend(Self::visit_houses(start, &robo_dirs));
        houses.extend(Self::visit_houses(start, &santa_dirs));
        houses.len().to_result()
    }
}
