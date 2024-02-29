use std::collections::{HashSet, VecDeque};

use aoc_lib::graph::Graph;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution12;
impl Solution12 {
    fn parse(input: ProblemInput) -> Graph<String> {
        let mut graph = Graph::new();
        for edge in input.lines() {
            let (v1, v2) = edge.split_once('-').unwrap();
            graph.entry(v1.to_string()).or_default().insert(v2.to_string());
            graph.entry(v2.to_string()).or_default().insert(v1.to_string());
        }
        graph
    }

    fn count_paths(cavenet: &Graph<String>, allowed_double_visits: u16) -> u32 {
        let mut queue = VecDeque::from([("start", HashSet::from(["start"]), 0)]);
        let mut paths = 0;
        while let Some((cave, visited, double_visits)) = queue.pop_front() {
            for nb in &cavenet[cave] {
                // Path found
                if nb == "end" {
                    paths += 1;
                    continue;
                }

                // Check if neighbor can be visited
                let mut new_visited = visited.clone();
                let mut new_double_visits = double_visits;
                if nb.chars().all(|c| c.is_ascii_lowercase()) && !new_visited.insert(nb) {
                    if double_visits >= allowed_double_visits || nb == "start" {
                        continue;
                    }
                    new_double_visits += 1;
                }

                // Explore further (BFS)
                queue.push_back((nb, new_visited, new_double_visits));
            }
        }

        paths
    }
}

impl Solution for Solution12 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(19),
            ProblemResult::U32(4970),
            ProblemResult::U32(103),
            ProblemResult::U32(137948),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let cavenet = Self::parse(input);
        let paths = Self::count_paths(&cavenet, 0);
        paths.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let cavenet = Self::parse(input);
        let paths = Self::count_paths(&cavenet, 1);
        paths.to_result()
    }
}
