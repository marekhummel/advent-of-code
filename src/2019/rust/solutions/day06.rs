use aoc_lib::graph::{Graph, PathFinding};
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let system = parse(input);
    orbits("COM", &system, 0).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut system = parse(input);

    // Build full graph
    for center in system.clone().vertices() {
        let orbits = system.adjacent_vertices(&center);
        for orbit in orbits {
            system.add_edge(&orbit, &center, true);
        }
    }

    // Don't count start and end
    let shortest_path = system.astar_no_heuristic(&"YOU".to_string(), &"SAN".to_string());
    (shortest_path.unwrap().0 - 2).into()
}

fn parse(input: ProblemInput) -> Graph<String> {
    let mut tree = Graph::empty();
    input.lines().into_iter().for_each(|line| {
        let (center, orbit) = line.split_once(')').unwrap();
        tree.add_edge(&center.to_string(), &orbit.to_string(), true);
    });

    tree
}

fn orbits(obj: &str, tree: &Graph<String>, depth: u32) -> u32 {
    let mut total = depth;
    for orbit in tree.adjacent_vertices(&obj.to_string()) {
        total += orbits(&orbit, tree, depth + 1);
    }

    total
}
