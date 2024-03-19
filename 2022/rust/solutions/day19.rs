use std::collections::{BinaryHeap, HashMap};

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

type Blueprint = [[u16; 4]; 4];

pub struct Solution19;
impl Solution19 {
    fn parse(input: ProblemInput) -> Vec<Blueprint> {
        let mineral_idx: HashMap<_, _> = ["ore", "clay", "obsidian"].iter().zip(0..).collect();
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (_, blueprint) = l.split_once(": ").unwrap();
                let robot_strs = blueprint.split('.');
                let costs_strs = robot_strs.filter_map(|rs| rs.split_once("costs ").map(|c| c.1.split(" and ")));

                let mut blueprint = [[0; 4]; 4];
                for (robot, costs) in costs_strs.enumerate() {
                    for cost in costs {
                        let (num, mineral) = cost.split_once(' ').unwrap();
                        blueprint[robot][mineral_idx[&mineral]] += num.parse::<u16>().unwrap();
                    }
                }
                blueprint
            })
            .collect()
    }

    fn emulate(blueprint: &Blueprint, max_time: u16) -> u16 {
        let max_needed = [0, 1, 2].map(|m| blueprint.iter().map(|r| r[m]).max().unwrap());

        let mut queue = BinaryHeap::from([(0, [1, 0, 0, 0], [0u16, 0, 0, 0], 0u16)]);
        let mut seen = HashMap::new();
        let mut most_geodes = 0;
        while let Some((_, robots, minerals, time)) = queue.pop() {
            // Last iteration
            if time == max_time {
                most_geodes = most_geodes.max(minerals[3]);
                continue;
            }
            seen.insert(robots, time);

            // Try to build each robot
            let mut can_build_all = true;
            for r in 0..4 {
                // Check if resources are available
                if !(0..4).all(|i| blueprint[r][i] <= minerals[i]) {
                    can_build_all = false;
                    continue;
                }

                // Don't build more robots than needed
                if r != 3 && robots[r] >= max_needed[r] {
                    continue;
                }

                let mut new_robots = robots;
                new_robots[r] += 1;

                // If we had the same amount of robots at an earlier time, this is not worth persuing
                if let Some(prev_time) = seen.get(&new_robots) {
                    if *prev_time <= time + 1 {
                        continue;
                    }
                }

                // Mine and build new robot (if we can even improve our best result)
                let new_minerals = [0, 1, 2, 3].map(|i| minerals[i] + robots[i] - blueprint[r][i]);
                let max_possible = Self::max_possible_geodes(&new_robots, &new_minerals, max_time - time);
                if max_possible > most_geodes {
                    queue.push((max_possible, new_robots, new_minerals, time + 1));
                }
            }

            // If we can't build all robots yet, it might be worth to save
            if !can_build_all {
                // Only proceed, if we can even improve our best result
                let new_minerals = [0, 1, 2, 3].map(|i| minerals[i] + robots[i]);
                let max_possible = Self::max_possible_geodes(&robots, &new_minerals, max_time - time);
                if max_possible > most_geodes {
                    queue.push((max_possible, robots, new_minerals, time + 1));
                }
            }
        }

        most_geodes
    }

    /// Maximum possible collected geodes, based on current state.
    /// Assumes that every minute a new geode robot is build and farms.
    fn max_possible_geodes(robots: &[u16; 4], minerals: &[u16; 4], rem_time: u16) -> u16 {
        minerals[3] + rem_time * robots[3] + rem_time * (rem_time + 1) / 2
    }
}

impl Solution for Solution19 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U16(33),
            ProblemResult::U16(1081),
            ProblemResult::U16(3472),
            ProblemResult::U16(2415),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let blueprints = Self::parse(input);
        let most_geodes = blueprints.iter().map(|bp| Self::emulate(bp, 24));
        let quality_levels = most_geodes.enumerate().map(|(id, geodes)| (id as u16 + 1) * geodes);
        quality_levels.sum::<u16>().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let blueprints = Self::parse(input).into_iter().take(3).collect_vec();
        let most_geodes = blueprints.iter().map(|bp| Self::emulate(bp, 32));
        most_geodes.product::<u16>().to_result()
    }
}
