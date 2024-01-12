use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum HexDirection {
    North,
    NorthWest,
    NorthEast,
    South,
    SouthWest,
    SouthEast,
}

#[derive(Debug, Clone)]
struct HexIndex {
    x: i128,
    y: i128,
}

// Use double-height coordinates
// https://www.redblobgames.com/grids/hexagons/
impl HexIndex {
    fn step(&self, dir: HexDirection) -> HexIndex {
        match dir {
            HexDirection::North => HexIndex {
                x: self.x,
                y: self.y - 2,
            },
            HexDirection::NorthWest => HexIndex {
                x: self.x - 1,
                y: self.y - 1,
            },
            HexDirection::NorthEast => HexIndex {
                x: self.x + 1,
                y: self.y - 1,
            },
            HexDirection::South => HexIndex {
                x: self.x,
                y: self.y + 2,
            },
            HexDirection::SouthWest => HexIndex {
                x: self.x - 1,
                y: self.y + 1,
            },
            HexDirection::SouthEast => HexIndex {
                x: self.x + 1,
                y: self.y + 1,
            },
        }
    }

    fn dist(&self, other: &HexIndex) -> u128 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx + dy.saturating_sub(dx) / 2
    }
}

pub struct Solution11;
impl Solution11 {
    fn parse(input: ProblemInput) -> Vec<HexDirection> {
        input
            .string()
            .split(',')
            .map(|d| match d {
                "n" => HexDirection::North,
                "nw" => HexDirection::NorthWest,
                "ne" => HexDirection::NorthEast,
                "s" => HexDirection::South,
                "sw" => HexDirection::SouthWest,
                "se" => HexDirection::SouthEast,
                _ => unreachable!(),
            })
            .collect_vec()
    }
}

impl Solution for Solution11 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let path = Self::parse(input);

        let start = HexIndex { x: 0, y: 0 };
        let mut target = start.clone();
        for dir in path {
            target = target.step(dir);
        }

        start.dist(&target).into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let path = Self::parse(input);

        let start = HexIndex { x: 0, y: 0 };
        let mut target = start.clone();
        let mut max_dist = 0;
        for dir in path {
            target = target.step(dir);
            max_dist = max_dist.max(start.dist(&target))
        }

        max_dist.into_some()
    }
}
