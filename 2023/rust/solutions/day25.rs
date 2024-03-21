use std::collections::{HashMap, HashSet};

use aoc_lib::graph::{Graph, PathFinding};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution25;
impl Solution25 {
    fn parse(input: ProblemInput) -> Graph<String> {
        let mut connections = HashMap::new();
        for line in input.lines() {
            let (left, right) = line.split_once(':').unwrap();
            let src = left.trim().to_string();
            let targets = right.split_whitespace().map(|t| t.trim().to_string()).collect_vec();

            targets.iter().for_each(|t| {
                _ = connections
                    .entry(t.clone())
                    .or_insert(HashSet::new())
                    .insert(src.clone());
            });
            connections.entry(src).or_insert(HashSet::new()).extend(targets);
        }

        connections.into_iter().collect()
    }

    // Count edges for all pairs of shortest paths (apsp)
    fn count_edges_apsp(graph: &mut Graph<String>) -> HashMap<(String, String), i32> {
        graph
            .vertices()
            .iter()
            .map(|v| {
                let mut edge_counts = HashMap::new();
                let ssp = graph.dijkstra(v);
                for paths in ssp.values() {
                    for (a, b) in paths.iter().tuple_windows() {
                        *edge_counts.entry((a.min(b).clone(), a.max(b).clone())).or_insert(0) += 1;
                    }
                }
                edge_counts
            })
            .fold(HashMap::new(), |mut acc, edge_counts| {
                for (v, c) in edge_counts {
                    *acc.entry(v).or_insert(0) += c;
                }
                acc
            })
    }
}

impl Solution for Solution25 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(54),
            ProblemResult::USize(547080),
            ProblemResult::NoPartTwo,
            ProblemResult::NoPartTwo,
        ]
    }

    // Takes about 13 secs / 3 secs (with and without release)
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut connections = Self::parse(input);

        // Run dijkstra for all pairs of vertices. Since the given network is a small world network
        // with two major components and three "bridges", those bridges will be used across apsp more
        // than any other edges
        let edge_counts = Self::count_edges_apsp(&mut connections);

        // Find bridges and remove from graph
        let bridges = edge_counts.into_iter().sorted_by_key(|(_, count)| -count).take(3);
        for ((from, to), _) in bridges {
            connections.remove_edge(&from, &to, false);
        }

        // Find components and their sizes
        // let components = Self::find_components(&connections);
        let components = connections.components();
        assert_eq!(components.len(), 2);
        components.into_iter().map(|c| c.len()).product::<usize>().to_result()
    }

    fn solve_version02(&self, _input: ProblemInput, _is_sample: bool) -> ProblemResult {
        // Just a button press :)
        ProblemResult::NoPartTwo
    }
}
