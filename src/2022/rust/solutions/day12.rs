use aoc_lib::cartesian::{Grid, Index};
use aoc_lib::graph::{DynamicGraph, PathFinding};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution12;
impl Solution12 {
    fn create_graph(grid: &Grid<char>) -> DynamicGraph<Index> {
        let map = grid.map_elements(|c| match c {
            'S' => 0,
            'E' => 25,
            _ => *c as u8 - b'a',
        });

        // Build graph from peak to start
        DynamicGraph {
            adjacent: Box::new(move |idx: &Index| {
                idx.von_neumann_neighbors(map.size)
                    .into_iter()
                    .filter(|nb| *map.get(idx) <= *map.get(nb) + 1)
                    .map(|nb| (nb, 1))
                    .collect()
            }),
        }
    }
}

impl Solution for Solution12 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I64(31),
            ProblemResult::I64(484),
            ProblemResult::USize(29),
            ProblemResult::USize(478),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let grid = input.grid();
        let mut graph = Self::create_graph(&grid);
        let goal = grid.enumerate().find(|(_, c)| **c == 'E').unwrap().0;
        let start = grid.enumerate().find(|(_, c)| **c == 'S').unwrap().0;
        let steps = graph.astar_no_heuristic(&goal, &start).unwrap().0;

        steps.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let grid = input.grid();
        let mut graph = Self::create_graph(&grid);
        let goal = grid.enumerate().find(|(_, c)| **c == 'E').unwrap().0;
        let starts = grid.enumerate().filter(|(_, c)| ['S', 'a'].contains(c)).map(|(i, _)| i);

        let shortest_paths = graph.dijkstra(&goal);
        let trails = starts.filter_map(|start| shortest_paths.get(&start).map(|p| p.len() - 1));
        trails.min().unwrap().to_result()
    }
}
