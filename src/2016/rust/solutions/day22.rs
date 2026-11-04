use aoc_lib::cartesian::{Grid, Index};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
struct Node {
    idx: Index,
    size: u16,
    used: u16,
    avail: u16,
}

pub struct Solution22;
impl Solution22 {
    fn parse(input: ProblemInput) -> Grid<Node> {
        let rgx = Regex::new(r"^\/dev\/grid\/node-x(?P<x>\d+)-y(?P<y>\d+)\s+(?P<size>\d+)T\s+(?P<used>\d+)T\s+(?P<avail>\d+)T\s+(?:\d+)%$").unwrap();
        let nodes = input
            .lines()
            .into_iter()
            .flat_map(|l| {
                rgx.captures(&l).map(|captures| {
                    let i = captures.name("x").unwrap().as_str().parse().unwrap();
                    let j = captures.name("y").unwrap().as_str().parse().unwrap();
                    let size = captures.name("size").unwrap().as_str().parse().unwrap();
                    let used = captures.name("used").unwrap().as_str().parse().unwrap();
                    let avail = captures.name("avail").unwrap().as_str().parse().unwrap();

                    Node {
                        idx: Index { i, j },
                        size,
                        used,
                        avail,
                    }
                })
            })
            .collect_vec();

        Grid::new(
            nodes
                .into_iter()
                .into_group_map_by(|n| n.idx.j)
                .into_iter()
                .sorted_by_key(|(j, _)| *j)
                .map(|(_, nodes)| nodes.into_iter().sorted_by_key(|n| n.idx.i).collect_vec())
                .collect_vec(),
        )
    }

    #[allow(dead_code)]
    fn print_grid(nodes: &Grid<Node>) {
        nodes.print(|idx, n| {
            if idx == (Index::new(0, 0)) {
                "S"
            } else if idx == (Index::new(nodes.size.width - 1, 0)) {
                "G"
            } else if n.used == 0 {
                "_"
            } else if nodes.iter().all(|b| n.used <= b.size) {
                "."
            } else {
                "#"
            }
        });
    }
}

impl Solution for Solution22 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(7),
            ProblemResult::USize(901),
            ProblemResult::USize(7),
            ProblemResult::USize(238),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let nodes = Self::parse(input);
        nodes
            .iter()
            .permutations(2)
            .map(|perm| perm.into_iter().collect_tuple().unwrap())
            .filter(|(a, b)| a.used > 0 && a.used <= b.avail)
            .count()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let nodes = Self::parse(input);
        // Self::print_grid(&nodes);

        // Assumptions (print grid and see):
        // 1. Most nodes are interchangeable, meaning that their data used can fit in any other node.
        // 2. The few nodes which data can't be transferred to the rest are forming a "wall" in one single row of the grid.
        // 3. There is a single node with no data, below (higher y) the wall.

        let mut total_swaps = 0;

        // 1. Find closest open spot in wall.
        let smallest_node_size = nodes.iter().map(|n| n.size).min().unwrap();
        let empty_node = nodes.iter().find(|n| n.used == 0).unwrap().idx;
        let wall_row = nodes
            .rows
            .iter()
            .find(|row| row.iter().any(|n| n.used > smallest_node_size))
            .unwrap();
        let best_spot = wall_row
            .iter()
            .filter(|n| n.used <= smallest_node_size)
            .min_by_key(|s| s.idx.i.abs_diff(empty_node.i))
            .unwrap();

        // 2. Move empty node upwards through that spot into the top row just left of G.
        total_swaps += empty_node.i.abs_diff(best_spot.idx.i);
        total_swaps += empty_node.j;
        total_swaps += (nodes.size.width - 2) - best_spot.idx.i;

        // 3. Repeat until G is in x = 1: Swap _ with G, then move empty node around to the left of G again (4 swaps)
        let steps_left = nodes.size.width - 2;
        total_swaps += steps_left * (1 + 4);

        // 4. Single swap from G into S.
        total_swaps += 1;

        // Finished
        total_swaps.to_result()
    }
}
