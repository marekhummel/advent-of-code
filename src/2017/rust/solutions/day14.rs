use aoc_lib::cartesian::Grid;
use aoc_lib::graph::Graph;
use aoc_lib::math::bits;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use aoc_lib::specific::knothash::KnotHash;
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let key = input.string();

    let mut used = 0;
    for i in 0..128 {
        let mut knot = KnotHash::new(&format!("{key}-{i}"));
        used += knot.hash().into_iter().map(|v| v.count_ones()).sum::<u32>();
    }

    used.into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let key = input.string();

    let mut rows = Vec::new();
    for i in 0..128 {
        let mut knot = KnotHash::new(&format!("{key}-{i}"));
        let row = knot.hash().iter().flat_map(|d| bits::<8>(*d as u128)).collect_vec();
        rows.push(row);
    }

    let grid = Grid::new(rows);
    let graph: Graph<_> = grid
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

    let components = graph.components();
    components.len().into()
}
