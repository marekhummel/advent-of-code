use aoc_lib::graph::Graph;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution09;
impl Solution09 {
    fn parse(input: ProblemInput) -> Graph<String> {
        let mut graph = Graph::empty();

        for l in input.lines() {
            let (route, distance_str) = l.split_once(" = ").unwrap();
            let (from, to) = route.trim().split_once(" to ").unwrap();
            let dist = distance_str.trim().parse().unwrap();

            graph.add_weighted_edge(&from.trim().to_string(), &to.trim().to_string(), dist, false);
        }

        graph
    }

    fn route_lenghts(graph: &Graph<String>) -> impl Iterator<Item = i64> + '_ {
        let vertices = graph.vertices();
        let num_vertices = vertices.len();
        vertices.into_iter().permutations(num_vertices).map(|route| {
            route
                .into_iter()
                .tuple_windows()
                .map(|(from, to)| graph.get_weight(&from, &to))
                .sum::<i64>()
        })
    }
}

impl Solution for Solution09 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I64(605),
            ProblemResult::I64(117),
            ProblemResult::I64(982),
            ProblemResult::I64(909),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let graph = Self::parse(input);
        Self::route_lenghts(&graph).min().unwrap().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let graph = Self::parse(input);
        Self::route_lenghts(&graph).max().unwrap().to_result()
    }
}
