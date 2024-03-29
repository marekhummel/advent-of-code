use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::intcode::Program;

pub struct Solution15;
impl Solution15 {
    fn find_oxygen_system(mut droid: Program) -> (usize, Position, HashMap<Position, bool>) {
        let directions = [Direction::North, Direction::South, Direction::West, Direction::East];

        let mut map = HashMap::from([(Position::zero(), true)]);
        let mut pos = Position::zero();
        let mut path = Vec::new();

        loop {
            let mut progress = false;
            for (dir_val, dir) in directions.iter().enumerate() {
                let new_pos = pos.advance_by(*dir, 1);
                if let Entry::Vacant(e) = map.entry(new_pos) {
                    droid.input.push_back(dir_val as i128 + 1);
                    let status = droid.execute_until_output().unwrap();
                    match status {
                        0 => _ = e.insert(false),
                        1 => {
                            e.insert(true);
                            pos = new_pos;
                            path.push(pos);
                            progress = true;
                        }
                        2 => {
                            e.insert(true);
                            path.push(new_pos);
                            return (path.len(), new_pos, map);
                        }
                        _ => unreachable!(),
                    }
                }
            }

            // Backtrack
            if !progress {
                path.pop().expect("No oxygen system found");
                let last_pos = path.last().unwrap();
                let dir = pos.get_direction_to(*last_pos);
                let command = directions.iter().position(|d| *d == dir).unwrap() as i128 + 1;
                droid.input.push_back(command);
                assert_eq!(droid.execute_until_output().unwrap(), 1);
                pos = *last_pos;
            }
        }
    }
}

impl Solution for Solution15 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::NoSample,
            ProblemResult::USize(298),
            ProblemResult::NoSample,
            ProblemResult::I32(346),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let droid = Program::init(&input.string());
        let (dist, _, _) = Self::find_oxygen_system(droid);
        dist.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        // Run droid
        let droid = Program::init(&input.string());
        let (_, oxy_sys, map) = Self::find_oxygen_system(droid);

        // Flood fill
        let mut filled = HashSet::new();
        let mut frontiers = HashSet::from([oxy_sys]);
        for min in 0.. {
            let mut new_frontiers = HashSet::new();

            for frontier in frontiers {
                for dir in Direction::compass() {
                    let adjacent = frontier.advance_by(dir, 1);
                    if *map.get(&adjacent).unwrap_or(&false) && !filled.contains(&adjacent) {
                        new_frontiers.insert(adjacent);
                        filled.insert(adjacent);
                    }
                }
            }

            if new_frontiers.is_empty() {
                return min.to_result();
            }

            frontiers = new_frontiers;
        }

        unreachable!()
    }
}
