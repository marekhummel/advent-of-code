use aoc_lib::graph::Graph;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let graph = parse(input);
    route_lenghts(&graph).min().unwrap().into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let graph = parse(input);
    route_lenghts(&graph).max().unwrap().into()
}

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
