use std::collections::{HashSet, VecDeque};

use aoc_lib::cartesian::{Grid, Index};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution09;
impl Solution09 {
    fn find_low_points(height_map: &Grid<u8>) -> impl Iterator<Item = (Index, &u8)> {
        height_map.enumerate().filter(|(idx, height)| {
            idx.von_neumann_neighbors(height_map.size)
                .into_iter()
                .map(|nb| height_map.get(&nb))
                .all(|nbh| nbh > height)
        })
    }
}

impl Solution for Solution09 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(15),
            ProblemResult::U32(491),
            ProblemResult::USize(1134),
            ProblemResult::USize(1075536),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let height_map = input.grid().map_elements(|c| *c as u8 - b'0');

        let low_points = Self::find_low_points(&height_map);
        let risk_sum = low_points.map(|(_, height)| *height as u32 + 1).sum::<u32>();
        risk_sum.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let height_map = input.grid().map_elements(|c| *c as u8 - b'0');
        let low_points = Self::find_low_points(&height_map);

        // Create basins with BFS over each low point
        let mut basins = Vec::new();
        for (depression, _) in low_points {
            let mut basin = HashSet::from([depression]);
            let mut frontiers = VecDeque::from([depression]);
            while let Some(frontier) = frontiers.pop_front() {
                for nb in frontier.von_neumann_neighbors(height_map.size) {
                    if !basin.contains(&nb) && *height_map.get(&nb) != 9 {
                        // nb_height > height is true per definition
                        basin.insert(nb);
                        frontiers.push_back(nb);
                    }
                }
            }

            basins.push(basin);
        }

        // Per definition we know that all positions will be in one basin, and also that the basins never overlap (due to the 9s).
        let largest_three = basins.into_iter().map(|b| b.len()).sorted().rev().take(3);
        largest_three.product::<usize>().to_result()
    }
}
