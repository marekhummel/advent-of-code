use aoc_lib::cartesian::Grid;
use aoc_lib::graph;
use aoc_lib::math::bits;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::knothash::KnotHash;
use itertools::Itertools;

pub struct Solution14;
impl Solution14 {}

impl Solution for Solution14 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let key = input.string();

        let mut used = 0;
        for i in 0..128 {
            let mut knot = KnotHash::new(&format!("{key}-{i}"));
            used += knot.hash().into_iter().map(|v| v.count_ones()).sum::<u32>();
        }

        used.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let key = input.string();

        let mut rows = Vec::new();
        for i in 0..128 {
            let mut knot = KnotHash::new(&format!("{key}-{i}"));
            let row = knot.hash().iter().flat_map(|d| bits::<8>(*d as u128)).collect_vec();
            rows.push(row);
        }

        let grid = Grid::new(rows);
        let graph = grid
            .enumerate()
            .filter(|(_, used)| **used)
            .map(|(idx, _)| {
                (
                    idx,
                    idx.von_neumann_neighbors(grid.size)
                        .into_iter()
                        .filter(|n| *grid.get(n))
                        .collect(),
                )
            })
            .collect();

        let components = graph::components(&graph);
        components.len().to_result()
    }
}
