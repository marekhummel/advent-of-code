use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_lib::graph;
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;
use rayon::iter::*;

pub struct Solution25;
impl Solution25 {
    fn parse(input: ProblemInput) -> HashMap<String, HashSet<String>> {
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

        connections
    }

    // Count edges for all pairs of shortest paths (apsp)
    fn count_edges_apsp(graph: &HashMap<String, HashSet<String>>) -> HashMap<(String, String), i32> {
        graph
            .keys()
            .collect_vec()
            .par_iter()
            .map(|v| {
                let mut edge_counts = HashMap::new();
                let ssp = graph::dijkstra(graph, v);
                for paths in ssp.values() {
                    for (a, b) in paths.iter().tuple_windows() {
                        *edge_counts.entry((a.min(b).clone(), a.max(b).clone())).or_insert(0) += 1;
                    }
                }
                edge_counts
            })
            .reduce(HashMap::new, |mut acc, edge_counts| {
                for (v, c) in edge_counts {
                    *acc.entry(v).or_insert(0) += c;
                }
                acc
            })
    }

    fn find_components(graph: &HashMap<String, HashSet<String>>) -> Vec<HashSet<String>> {
        let mut components = graph.keys().map(|v| HashSet::from([v.clone()])).collect_vec();

        for (v, neighbors) in graph.iter() {
            for u in neighbors {
                let v_comp = components.iter().position(|c| c.contains(v)).unwrap();
                let u_comp = components.iter().position(|c| c.contains(u)).unwrap();

                if v_comp != u_comp {
                    let other_comp = components[u_comp].clone();
                    components.get_mut(v_comp).unwrap().extend(other_comp);
                    components.remove(u_comp);
                }
            }
        }

        components
    }
}

impl Solution for Solution25 {
    // Takes about 13 secs / 3 secs (with and without release)
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut connections = Self::parse(input);

        // Run dijkstra for all pairs of vertices. Since the given network is a small world network
        // with two major components and three "bridges", those bridges will be used across apsp more
        // than any other edges
        let edge_counts = Self::count_edges_apsp(&connections);

        // Find bridges and remove from graph
        let bridges = edge_counts.into_iter().sorted_by_key(|(_, count)| -count).take(3);
        for ((from, to), _) in bridges {
            connections.get_mut(&from).unwrap().remove(&to);
            connections.get_mut(&to).unwrap().remove(&from);
        }

        // Find components and their sizes
        let components = Self::find_components(&connections);
        assert_eq!(components.len(), 2);
        components.into_iter().map(|c| c.len()).product::<usize>().into_some()
    }

    fn solve_version02(&self, _input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        // Just a button press :)
        None
    }
}
