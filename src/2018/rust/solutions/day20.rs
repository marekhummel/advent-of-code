use std::collections::{HashMap, HashSet, VecDeque};

use aoc_lib::cartesian::Position;
use aoc_lib::graph::Graph;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution20;
impl Solution20 {
    fn build_map<I: Iterator<Item = char>>(
        regex: &mut I,
        map: &mut Graph<Position>,
        initial: HashSet<Position>,
    ) -> HashSet<Position> {
        let mut current = initial.clone();
        let mut new_positions = HashSet::new();

        while let Some(ch) = regex.next() {
            match ch {
                'N' | 'E' | 'S' | 'W' => {
                    // Pass through door and mark in map
                    let dir = ch.try_into().unwrap();

                    let mut next_pos = HashSet::new();
                    for pos in current {
                        let next = pos.advance_by(dir, 1);
                        map.add_edge(&pos, &next, false);
                        next_pos.insert(next);
                    }
                    current = next_pos;
                }
                '|' => {
                    // Current "or"-branch is handled, restart with initial positiions
                    new_positions.extend(current);
                    current = initial.clone();
                }
                '(' => {
                    // Recurse
                    current = Self::build_map(regex, map, current);
                }
                ')' => break,
                _ => unreachable!(),
            }
        }

        new_positions.extend(current);
        new_positions
    }

    fn dists(map: &Graph<Position>) -> HashMap<Position, u32> {
        // Simple bfs to find distance to each room
        let mut queue = VecDeque::from([(Position::zero(), 0)]);
        let mut dists = HashMap::new();

        while let Some((pos, dist)) = queue.pop_front() {
            if dists.contains_key(&pos) {
                continue;
            }

            dists.insert(pos, dist);

            for next in map.adjacent_vertices(&pos) {
                queue.push_back((next, dist + 1));
            }
        }

        dists
    }
}

impl Solution for Solution20 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(23),
            ProblemResult::U32(3725),
            ProblemResult::USize(0),
            ProblemResult::USize(8541),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let input_str = input.string();
        let mut regex = input_str.trim_start_matches('^').trim_end_matches('$').chars();

        let mut map = Graph::empty();
        Self::build_map(&mut regex, &mut map, HashSet::from([Position::zero()]));

        let dists = Self::dists(&map);
        dists.values().max().unwrap().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let input_str = input.string();
        let mut regex = input_str.trim_start_matches('^').trim_end_matches('$').chars();

        let mut map = Graph::empty();
        Self::build_map(&mut regex, &mut map, HashSet::from([Position::zero()]));

        let dists = Self::dists(&map);
        dists.values().filter(|dist| **dist >= 1000).count().to_result()
    }
}
