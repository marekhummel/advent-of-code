use std::collections::{HashMap, HashSet};

use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

type Rule = ([usize; 3], Direction);

pub struct Solution23;
impl Solution23 {
    fn parse(input: ProblemInput) -> HashSet<Position> {
        input
            .grid()
            .enumerate()
            .filter(|(_, c)| **c == '#')
            .map(|(idx, _)| idx.into())
            .collect()
    }

    fn round(elves: &mut HashSet<Position>, rules: &mut [Rule]) -> bool {
        let proposals: HashMap<_, _> = elves.iter().map(|e| (*e, Self::proposal(e, elves, rules))).collect();
        if proposals.values().all(|(_, idle)| *idle) {
            return false;
        }

        let target_counts = proposals.values().map(|(t, _)| *t).counts();
        for (elf, (target, idle)) in proposals {
            if !idle && target_counts[&target] == 1 {
                elves.remove(&elf);
                elves.insert(target);
            }
        }

        rules.rotate_left(1);
        true
    }

    fn proposal(pos: &Position, elves: &HashSet<Position>, rules: &[Rule]) -> (Position, bool) {
        let neighbors = pos.moore_neighbors();
        if neighbors.iter().all(|nb| !elves.contains(nb)) {
            return (*pos, true);
        }

        let proposed_dir = rules
            .iter()
            .find(|(idcs, _)| idcs.iter().all(|i| !elves.contains(&neighbors[*i])))
            .map_or(Direction::None, |(_, dir)| *dir);

        (pos.advance_by(proposed_dir, 1), false)
    }

    /// Assuming Moore neighbors in order: N NW E NE S SE W SW
    fn proposal_rules() -> Vec<([usize; 3], Direction)> {
        vec![
            ([0, 1, 3], Direction::North),
            ([4, 5, 7], Direction::South),
            ([1, 6, 7], Direction::West),
            ([2, 3, 5], Direction::East),
        ]
    }
}

impl Solution for Solution23 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I128(110),
            ProblemResult::I128(4109),
            ProblemResult::U16(20),
            ProblemResult::U16(1055),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut elves = Self::parse(input);
        let mut rules = Self::proposal_rules();

        (0..10).for_each(|_| _ = Self::round(&mut elves, &mut rules));

        let (x_min, x_max) = elves.iter().map(|p| p.x).minmax().into_option().unwrap();
        let (y_min, y_max) = elves.iter().map(|p| p.y).minmax().into_option().unwrap();
        let area = (x_max - x_min + 1) * (y_max - y_min + 1);
        (area - elves.len() as i128).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut elves = Self::parse(input);
        let mut rules = Self::proposal_rules();

        let round = (1u16..).find(|_| !Self::round(&mut elves, &mut rules)).unwrap();
        round.to_result()
    }
}
