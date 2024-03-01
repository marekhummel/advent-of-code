use aoc_lib::graph::Graph;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution06;
impl Solution06 {
    fn parse(input: ProblemInput) -> Graph<String> {
        let mut tree = Graph::empty();
        input.lines().into_iter().for_each(|line| {
            let (center, orbit) = line.split_once(')').unwrap();
            tree.add_edge(&center.to_string(), &orbit.to_string(), true);
        });

        tree
    }

    fn orbits(obj: &str, tree: &Graph<String>, depth: u32) -> u32 {
        let mut orbits = depth;
        for orbit in tree.adjacent_vertices(&obj.to_string()) {
            orbits += Self::orbits(&orbit, tree, depth + 1);
        }

        orbits
    }
}

impl Solution for Solution06 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(42),
            ProblemResult::U32(344238),
            ProblemResult::I64(4),
            ProblemResult::I64(436),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let system = Self::parse(input);
        Self::orbits("COM", &system, 0).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut system = Self::parse(input);

        // Build full graph
        for center in system.clone().vertices() {
            let orbits = system.adjacent_vertices(&center);
            for orbit in orbits {
                system.add_edge(&orbit, &center, true);
            }
        }

        // Don't count start and end
        let shortest_path = system.astar_no_heuristic(&"YOU".to_string(), &"SAN".to_string());
        (shortest_path.unwrap().0 - 2).to_result()
    }
}
