use aoc_lib::cartesian::{Direction, Grid, Index};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Ground {
    Dirt,
    Clay,
    Flowing,
    Water,
}

impl Ground {
    fn is_solid(&self) -> bool {
        *self == Ground::Clay || *self == Ground::Water
    }

    fn is_water(&self) -> bool {
        *self == Ground::Flowing || *self == Ground::Water
    }
}

pub struct Solution17;
impl Solution17 {
    fn parse(input: ProblemInput) -> Vec<(usize, usize, usize, bool)> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (fixed, range) = l.split_once(',').unwrap();
                let (fixed_dim, fixed_value) = fixed.split_once('=').unwrap();
                let range_value = range.split_once('=').unwrap().1;
                let (low, high) = range_value.split_once("..").unwrap();
                (
                    fixed_value.parse().unwrap(),
                    low.parse().unwrap(),
                    high.parse().unwrap(),
                    fixed_dim == "x",
                )
            })
            .collect()
    }

    fn create_ground_grid(clay: Vec<(usize, usize, usize, bool)>) -> (Grid<Ground>, (usize, usize)) {
        let xs = clay.iter().flat_map(|&(f, l, h, x)| if x { [f, f] } else { [l, h] });
        let ys = clay.iter().flat_map(|&(f, l, h, x)| if !x { [f, f] } else { [l, h] });
        let (min_x, max_x) = xs.chain([500]).minmax().into_option().unwrap();
        let (min_y, max_y) = ys.minmax().into_option().unwrap();

        let mut ground = Grid::new(vec![vec![Ground::Dirt; max_x - min_x + 3]; max_y + 1]);
        for (fixed, low, high, is_vertical) in clay {
            for range in low..=high {
                let (i, j) = if is_vertical { (fixed, range) } else { (range, fixed) };
                ground.set(&Index { i: i - min_x + 1, j }, Ground::Clay);
            }
        }

        (ground, (min_x, min_y))
    }

    fn flow(ground: &mut Grid<Ground>, start: Index) {
        let mut idx = start;

        loop {
            ground.set(&idx, Ground::Flowing);
            let Some(down) = idx.advance_check(Direction::South, ground.size) else {
                return;
            };

            match ground.get(&down) {
                Ground::Dirt => idx = down,
                Ground::Clay | Ground::Water => break,
                Ground::Flowing => return, // don't rerun existing flows
            }
        }

        loop {
            // Flow left as far as possible
            let mut idx_left = idx;
            let mut box_left = false;
            while ground.get(&idx_left.advance(Direction::South)).is_solid() {
                ground.set(&idx_left, Ground::Flowing);

                let left = idx_left.advance(Direction::West);
                if ground.get(&left).is_solid() {
                    box_left = true;
                    break;
                }

                idx_left = left;
            }

            // Flow right as far as possible
            let mut idx_right = idx;
            let mut box_right = false;
            while ground.get(&idx_right.advance(Direction::South)).is_solid() {
                ground.set(&idx_right, Ground::Flowing);

                let right = idx_right.advance(Direction::East);
                if ground.get(&right).is_solid() {
                    box_right = true;
                    break;
                }

                idx_right = right;
            }

            // If both sides are contained, we are in a bucket (fill until overflow)
            if box_left && box_right {
                let j = idx.j;
                for i in idx_left.i..=idx_right.i {
                    ground.set(&Index { i, j }, Ground::Water);
                }
                idx = idx.advance(Direction::North);
                continue;
            }

            // If not, we overflow either left or right, abort loop
            if !box_left {
                Self::flow(ground, idx_left);
            }
            if !box_right {
                Self::flow(ground, idx_right);
            }

            break;
        }
    }
}

impl Solution for Solution17 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let clay = Self::parse(input);
        let (mut ground, (min_x, min_y)) = Self::create_ground_grid(clay);

        let start = Index::new(501 - min_x, 0);
        Self::flow(&mut ground, start);

        (ground.iter().filter(|g| g.is_water()).count() - min_y).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let clay = Self::parse(input);
        let (mut ground, (min_x, _)) = Self::create_ground_grid(clay);

        let start = Index::new(501 - min_x, 0);
        Self::flow(&mut ground, start);

        ground.iter().filter(|g| **g == Ground::Water).count().to_result()
    }
}
