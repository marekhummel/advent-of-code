use std::collections::HashSet;

use aoc_lib::cartesian::{HexDirection, HexIndex};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution24;
impl Solution24 {
    fn parse(input: ProblemInput) -> Vec<Vec<HexDirection>> {
        input
            .lines()
            .into_iter()
            .map(|line| {
                let mut dirs = Vec::new();
                let mut i = 0;
                while i < line.len() {
                    // We rotate all directions, beucase struct support flat up grid, problem has pointy up
                    dirs.push(match &line[i..=i] {
                        d @ "n" | d @ "s" => {
                            i += 1;
                            match (d, &line[i..=i]) {
                                ("n", "w") => HexDirection::SouthWest,
                                ("n", "e") => HexDirection::NorthWest,
                                ("s", "w") => HexDirection::SouthEast,
                                ("s", "e") => HexDirection::NorthEast,
                                _ => unreachable!(),
                            }
                        }
                        "w" => HexDirection::South,
                        "e" => HexDirection::North,
                        _ => unreachable!(),
                    });
                    i += 1;
                }
                dirs
            })
            .collect()
    }

    fn flip_tiles(paths: &[Vec<HexDirection>]) -> HashSet<HexIndex> {
        let mut blacks = HashSet::new();
        for path in paths {
            let target = path.iter().fold(HexIndex { x: 0, y: 0 }, |idx, dir| idx.step(dir));

            if !blacks.contains(&target) {
                blacks.insert(target);
            } else {
                blacks.remove(&target);
            }
        }

        blacks
    }
}

impl Solution for Solution24 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(10),
            ProblemResult::USize(312),
            ProblemResult::USize(2208),
            ProblemResult::USize(3733),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let paths = Self::parse(input);
        let blacks = Self::flip_tiles(&paths);
        blacks.len().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let paths = Self::parse(input);
        let mut blacks = Self::flip_tiles(&paths);

        for _ in 0..100 {
            let mut new_blacks = HashSet::new();

            for tile in blacks.iter().flat_map(|b| b.neighbors()).unique() {
                let black_nb = tile.neighbors().iter().filter(|nb| blacks.contains(nb)).count();
                if black_nb == 2 || (blacks.contains(&tile) && black_nb == 1) {
                    new_blacks.insert(tile);
                }
            }

            blacks = new_blacks
        }

        blacks.len().to_result()
    }
}
