use std::collections::HashMap;

use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution09;
impl Solution09 {
    fn parse(input: ProblemInput) -> HashMap<String, HashMap<String, u64>> {
        let mut graph = HashMap::new();

        for l in input.lines() {
            let (route, distance_str) = l.split_once(" = ").unwrap();
            let (from, to) = route.trim().split_once(" to ").unwrap();
            let dist = distance_str.trim().parse::<u64>().unwrap();

            graph
                .entry(from.trim().to_string())
                .or_insert(HashMap::new())
                .insert(to.trim().to_string(), dist);

            graph
                .entry(to.trim().to_string())
                .or_insert(HashMap::new())
                .insert(from.trim().to_string(), dist);
        }

        graph
    }

    fn route_lenghts(graph: &HashMap<String, HashMap<String, u64>>) -> impl Iterator<Item = u64> + '_ {
        graph.keys().permutations(graph.len()).map(|route| {
            route
                .into_iter()
                .tuple_windows()
                .map(|(from, to)| graph[from][to])
                .sum::<u64>()
        })
    }
}

impl Solution for Solution09 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let graph = Self::parse(input);
        Self::route_lenghts(&graph).min().unwrap().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let graph = Self::parse(input);
        Self::route_lenghts(&graph).max().unwrap().to_result()
    }
}
