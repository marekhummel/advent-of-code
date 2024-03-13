use std::fmt::Display;
use std::ops::Range;

use aoc_lib::cartesian::Grid;
use aoc_lib::graph::{DynamicGraph, PathFinding};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

/// Image cave as list of indices. Start with the rooms first, from the innermost to the outermost.
/// Given 4 amphipods per type, room A is 0-3, B is 4-7 etc., with the highest one connecting to the hallway.
/// Hallway is 7 tiles wide (ignoring the spaces in front of rooms), increasing from left to right.
#[derive(Clone)]
struct Cave {
    amphipod_types: u8,
    room_depth: u8,
    hallway_start: u8,
    hallway_end: u8,
}

impl Cave {
    fn new(amphipod_types: u8, room_depth: u8) -> Self {
        Self {
            amphipod_types,
            room_depth,
            hallway_start: amphipod_types * room_depth,
            hallway_end: amphipod_types * room_depth + 6,
        }
    }

    fn room_indices(&self, room: u8) -> Range<u8> {
        let beginning = room * self.room_depth;
        beginning..beginning + self.room_depth
    }

    fn dist_to_hallway(&self, pos: u8) -> u8 {
        assert!(!self.is_in_hallway(pos));
        let room_pos = self
            .room_indices(self.get_room(pos))
            .nth(self.room_depth as usize - 1)
            .unwrap();
        room_pos + 1 - pos
    }

    fn dist_in_hallway(&self, from: u8, to: u8) -> u8 {
        assert!(self.is_in_hallway(from));
        assert!(self.is_in_hallway(to));
        let mut dist = to.abs_diff(from) * 2;
        if from == self.hallway_start || to == self.hallway_start {
            dist -= 1;
        }
        if from == self.hallway_end || to == self.hallway_end {
            dist -= 1;
        }
        dist
    }

    fn hallway_entry_points(&self, room: u8) -> (u8, u8) {
        let left = room + self.hallway_start + 1;
        let right = left + 1;
        (left, right)
    }

    fn is_in_hallway(&self, pos: u8) -> bool {
        pos >= self.hallway_start
    }

    fn get_room(&self, pos: u8) -> u8 {
        assert!(!self.is_in_hallway(pos));
        pos / self.room_depth
    }

    fn goal_config(&self) -> Config {
        let room = (1 << self.room_depth) - 1;
        let rooms = [0, 1, 2, 3].map(|i| room << (i * self.room_depth));
        Config {
            amphipods: rooms,
            room_depth: self.room_depth,
        }
    }

    /// Given the current pos of some amphipod and the config, find the list of (logically and physically)
    /// reachable positions. Note that only room -> hallway (and vice versa) moves are considered. Room -> Room can
    /// be achieved in two steps, moves within the hallway do not make sense.
    fn reachable(&self, amphipod: u8, pos: u8, config: &Config) -> Vec<(u8, i64)> {
        if self.is_in_hallway(pos) {
            // ** Move from hallway to room

            // If room still contains other amphipods, don't move
            let room = self.room_indices(amphipod);
            if room.into_iter().any(|r| config.contains_other_at(amphipod, r)) {
                return vec![];
            }

            // Find closest hallway pos to room
            let (left, right) = self.hallway_entry_points(amphipod);
            let hallway_target = [left, right].into_iter().min_by_key(|hp| hp.abs_diff(pos)).unwrap();

            // Check if path in hallway is clear
            let mut hallway_path = pos.min(hallway_target)..=pos.max(hallway_target);
            if hallway_path.any(|hp| hp != pos && config.contains_any_at(hp)) {
                return vec![];
            }

            // Find deepest pos in room
            let target = self
                .room_indices(amphipod)
                .find(|p| !config.contains_same_at(amphipod, *p))
                .unwrap();

            let dist = (self.dist_in_hallway(pos, hallway_target) + 1) + ((amphipod + 1) * self.room_depth - target);

            // When moving into a room, there is only one place to go to
            vec![(target, dist as i64)]
        } else {
            // ** Move from room to hallway

            // Only move if not already in right room
            let room = self.get_room(pos);
            if room == amphipod
                && self
                    .room_indices(room)
                    .filter(|rp| *rp < pos)
                    .all(|rp| config.contains_same_at(amphipod, rp))
            {
                return vec![];
            }

            // Check if leaving the room is possible
            if self
                .room_indices(room)
                .filter(|rp| *rp > pos)
                .any(|rp| config.contains_any_at(rp))
            {
                return vec![];
            }

            // Find reachable from hallway entrances
            let dist_to_hallway = self.dist_to_hallway(pos) as i64;
            let mut reachable = Vec::new();
            let (mut left, mut right) = self.hallway_entry_points(room);

            let mut left_steps = 1;
            while !config.contains_any_at(left) && left >= self.hallway_start {
                reachable.push((left, dist_to_hallway + left_steps));
                left -= 1;
                left_steps += if left > self.hallway_start { 2 } else { 1 };
            }

            let mut right_steps = 1;
            while !config.contains_any_at(right) && right <= self.hallway_end {
                reachable.push((right, dist_to_hallway + right_steps));
                right += 1;
                right_steps += if right < self.hallway_end { 2 } else { 1 };
            }

            reachable
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Config {
    amphipods: [u32; 4],
    room_depth: u8,
}

impl Config {
    fn from_grid(grid: &Grid<char>, room_depth: u8) -> Self {
        let pods = grid
            .map_elements(|c| *c as u8)
            .enumerate()
            .filter(|(_, c)| (b'A'..=b'D').contains(c))
            .map(|(idx, c)| (((idx.i - 3) / 2) as u8, room_depth + 1 - idx.j as u8, c - b'A'))
            .collect_vec();

        let ints = (0..4)
            .map(|ap| {
                pods.iter()
                    .filter(|(_, _, p)| *p == ap)
                    .map(|(room, depth, _)| room * room_depth + depth)
                    .fold(0, |pos, tile| pos | (1 << tile))
            })
            .collect_vec();

        Config {
            amphipods: ints.try_into().unwrap(),
            room_depth,
        }
    }

    fn move_amphipod(&mut self, amphipod: u8, from: u8, to: u8) {
        let usize_amphipod = amphipod as usize;
        assert_eq!(self.amphipods[usize_amphipod].count_ones() as u8, self.room_depth);
        self.amphipods[usize_amphipod] &= !(1 << from);
        self.amphipods[usize_amphipod] |= 1 << to;
        assert_eq!(self.amphipods[usize_amphipod].count_ones() as u8, self.room_depth);
    }

    fn positions(&self, amphipod: u8) -> Vec<u8> {
        (0..32)
            .filter(|p| (self.amphipods[amphipod as usize] >> p) & 1 == 1)
            .collect()
    }

    fn contains_same_at(&self, amphipod: u8, pos: u8) -> bool {
        (self.amphipods[amphipod as usize] >> pos) & 1 == 1
    }

    fn contains_any_at(&self, pos: u8) -> bool {
        let all = self.amphipods[0] | self.amphipods[1] | self.amphipods[2] | self.amphipods[3];
        (all >> pos) & 1 == 1
    }

    fn contains_other_at(&self, amphipod: u8, pos: u8) -> bool {
        let mut others = self.amphipods[0] | self.amphipods[1] | self.amphipods[2] | self.amphipods[3];
        others ^= self.amphipods[amphipod as usize];
        (others >> pos) & 1 == 1
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hallway = vec![(1, 1), (1, 2), (1, 4), (1, 6), (1, 8), (1, 10), (1, 11)];
        let rooms = (0..4)
            .flat_map(|r| (2..=self.room_depth + 1).rev().map(move |d| (d as usize, r * 2 + 3)))
            .collect_vec();
        let lookup = [rooms, hallway].concat();

        let mut lines = vec![
            "#############".chars().collect_vec(),
            "#...........#".chars().collect_vec(),
            "###.#.#.#.###".chars().collect_vec(),
            "  #.#.#.#.#  ".chars().collect_vec(),
            "  #.#.#.#.#  ".chars().collect_vec(),
            "  #.#.#.#.#  ".chars().collect_vec(),
            "  #########  ".chars().collect_vec(),
        ];

        if self.room_depth == 2 {
            lines.remove(4);
            lines.remove(4);
        }

        for amphipod in 0..4 {
            for pos in self.positions(amphipod) {
                let (y, x) = lookup[pos as usize];
                lines[y][x] = (amphipod + b'A') as char;
            }
        }

        writeln!(
            f,
            "{}",
            lines.into_iter().map(|l| l.into_iter().collect::<String>()).join("\n")
        )
    }
}

pub struct Solution23;
impl Solution23 {
    fn extend_grid(folded: &mut Grid<char>) {
        folded.rows.insert(3, "  #D#C#B#A#".chars().collect());
        folded.rows.insert(4, "  #D#B#A#C#".chars().collect());
    }

    fn create_config_graph(cave: Cave) -> DynamicGraph<Config> {
        // Use dynamic graph. No need to build the entire graph of config transitions.
        DynamicGraph {
            adjacent: Box::new(move |config: &Config| {
                let mut children = Vec::new();
                for amphipod in 0..cave.amphipod_types {
                    let energy = 10i64.pow(amphipod as u32);
                    for pos in config.positions(amphipod) {
                        for (nb, steps) in cave.reachable(amphipod, pos, config) {
                            // Add to config graph
                            let mut next_config = *config;
                            next_config.move_amphipod(amphipod, pos, nb);
                            children.push((next_config, energy * steps));
                        }
                    }
                }
                children
            }),
        }
    }

    fn compute_best_config_path(cave: &Cave, config_graph: &DynamicGraph<Config>, start: Config, goal: Config) -> i64 {
        let best_moves = config_graph.astar(&start, &goal, |current| {
            let mut diffs = 0;
            for amphipod in 0..cave.amphipod_types {
                let mut amphipod_diffs = 0;
                for pos in current.positions(amphipod) {
                    if !cave.is_in_hallway(pos) {
                        // ** In room

                        if cave.get_room(pos) == amphipod {
                            // Correct room already
                            // Has to vacate room to make space for others
                            if cave
                                .room_indices(amphipod)
                                .any(|p| p < pos && current.contains_other_at(amphipod, p))
                            {
                                amphipod_diffs += 2 * cave.dist_to_hallway(pos) as i64;
                            }
                        } else {
                            // Wrong room
                            let (in_left, in_right) = cave.hallway_entry_points(amphipod);
                            amphipod_diffs += cave.dist_to_hallway(pos) as i64;

                            let (out_left, out_right) = cave.hallway_entry_points(cave.get_room(pos));
                            amphipod_diffs += cave
                                .dist_in_hallway(out_left, in_right)
                                .min(cave.dist_in_hallway(out_right, in_left))
                                as i64;

                            let room = cave.room_indices(amphipod);
                            let highest = room
                                .into_iter()
                                .filter(|r| !current.contains_any_at(*r))
                                .max()
                                .unwrap_or_default();
                            amphipod_diffs += cave.dist_to_hallway(highest) as i64;
                        }
                    } else {
                        // In hallway

                        // Find dist to entry points
                        let (in_left, in_right) = cave.hallway_entry_points(amphipod);
                        amphipod_diffs += cave
                            .dist_in_hallway(pos, in_left)
                            .min(cave.dist_in_hallway(pos, in_right)) as i64;

                        // Enter room
                        let room = cave.room_indices(amphipod);
                        let highest = room.filter(|r| !current.contains_any_at(*r)).max().unwrap_or_default();
                        amphipod_diffs += cave.dist_to_hallway(highest) as i64;
                    }
                }

                diffs += amphipod_diffs * 10i64.pow(amphipod as u32);
            }

            diffs
        });

        best_moves.unwrap().0
    }
}

impl Solution for Solution23 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I64(12521),
            ProblemResult::I64(13520),
            ProblemResult::I64(44169),
            ProblemResult::I64(48708),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let cave = Cave::new(4, 2);

        let start = Config::from_grid(&input.grid(), cave.room_depth);

        let config_graph = Self::create_config_graph(cave.clone());
        let goal = cave.goal_config();
        let best_moves = Self::compute_best_config_path(&cave, &config_graph, start, goal);
        best_moves.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let cave = Cave::new(4, 4);

        let mut grid = input.grid();
        Self::extend_grid(&mut grid);
        let start = Config::from_grid(&grid, cave.room_depth);

        let config_graph = Self::create_config_graph(cave.clone());
        let goal = cave.goal_config();
        let best_moves = Self::compute_best_config_path(&cave, &config_graph, start, goal);
        best_moves.to_result()
    }
}
