use std::collections::{HashMap, HashSet};

use aoc_lib::graph;
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution06;
impl Solution06 {
    fn parse(input: ProblemInput) -> HashMap<String, HashSet<String>> {
        let mut tree = HashMap::new();
        input.lines().into_iter().for_each(|line| {
            let (center, orbit) = line.split_once(')').unwrap();
            _ = tree
                .entry(center.to_string())
                .or_insert(HashSet::new())
                .insert(orbit.to_string());
        });

        tree
    }

    fn orbits(obj: &str, tree: &HashMap<String, HashSet<String>>, depth: u32) -> u32 {
        let mut orbits = depth;
        if let Some(orbiting) = tree.get(obj) {
            for orbit in orbiting {
                orbits += Self::orbits(orbit, tree, depth + 1);
            }
        }

        orbits
    }
}

impl Solution for Solution06 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let system = Self::parse(input);
        Self::orbits("COM", &system, 0).into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut system = Self::parse(input);

        // Build full graph
        for (center, orbiting) in system.clone() {
            for orbit in orbiting {
                system.entry(orbit.to_string()).or_default().insert(center.to_string());
            }
        }

        let shortest_paths = graph::dijkstra(&system, &"YOU".to_string());
        let path = shortest_paths.get("SAN").unwrap();

        // Remove YOU and SAN from path (-2), and count edges between vertices (= vertices - 1)
        (path.len() - 3).into_some()
    }
}
