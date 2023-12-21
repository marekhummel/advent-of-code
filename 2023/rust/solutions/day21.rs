use std::collections::HashSet;

use aoc_lib::solution::Solution;
use aoc_lib::types::{Grid, IntoSome, ProblemInput, ProblemResult};
use aoc_lib::util::{Index, Position, Size};
use itertools::Itertools;

type BoolGrid = Grid<bool>;

pub struct Solution21;
impl Solution21 {
    fn parse(input: ProblemInput) -> (BoolGrid, Index) {
        let grid = input.grid();

        let start = grid
            .iter()
            .find_position(|row| row.iter().any(|&c| c == 'S'))
            .map(|(y, row)| Index {
                j: y,
                i: row.iter().position(|&c| c == 'S').unwrap(),
            })
            .unwrap();

        let bool_grid = grid
            .into_iter()
            .map(|row| row.into_iter().map(|c| c != '#').collect_vec())
            .collect_vec();

        (bool_grid, start)
    }

    fn walk(grid: &BoolGrid, current: &[Position], steps: usize) -> Vec<Position> {
        let size = Size::from_grid(grid);

        let mut positions = current.iter().cloned().collect_vec();
        for _ in 0..steps {
            let mut new_positions = HashSet::new();
            for pos in positions {
                let reached = pos
                    .von_neumann_neighbors(1)
                    .into_iter()
                    .filter(|p| *p.wrap_modular(size).grid_get(grid));
                new_positions.extend(reached);
            }

            positions = new_positions.into_iter().collect();
        }

        positions
    }

    #[allow(dead_code)]
    fn print(grid: &BoolGrid, positions: &HashSet<Index>) {
        let size = Size::from_grid(grid);
        #[allow(clippy::needless_range_loop)]
        for j in 0..size.height {
            for i in 0..size.width {
                match (grid[j][i], positions.contains(&Index { j, i })) {
                    (true, true) => print!("O"),
                    (true, false) => print!("."),
                    (false, true) => panic!(),
                    (false, false) => print!("#"),
                }
            }
            println!();
        }
        println!();
    }
}

impl Solution for Solution21 {
    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        let (grid, start) = Self::parse(input);
        let steps = 64;

        let plots = Self::walk(&grid, &[start.into()], steps);
        plots.len().into_some()
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        let (grid, start) = Self::parse(input);
        let size = Size::from_grid(&grid);
        let steps = 26501365;

        if size.width != 131 {
            return None; // Won't be able to solve sample case
        }

        // Lets note that the grid is 131 cells wide in both dimensions, the start point is exactly centered.
        // Further the middle row and column are completely free, meaning that from the start we can reach the first repitition
        // within 65 steps and the next ones every 131 steps.
        // Thus the final result will follow a quadratic function, and so its enough to find 3 points on that parabola to compute
        // its coefficients. Note that the x values are technically arbitrary, as long as we can compute the final x from the required
        // step count.

        // Thus compute the first three times the borders are crossed
        // Note: This takes like 20 secs
        let pos0 = Self::walk(&grid, &[start.into()], 65);
        let pos1 = Self::walk(&grid, &pos0, 131);
        let pos2 = Self::walk(&grid, &pos1, 131);

        // Collect the points for the parabola (note xs = (65, 196, 327) works fine too, then x = steps)
        let (x0, x1, x2) = (0, 1, 2);
        let (y0, y1, y2) = (pos0.len() as i128, pos1.len() as i128, pos2.len() as i128);

        // Define x and compute y with lagranges interpolation
        let x = steps / 131;
        let y = y0 * (x - x1) * (x - x2) / ((x0 - x1) * (x0 - x2))
            + y1 * (x - x0) * (x - x2) / ((x1 - x0) * (x1 - x2))
            + y2 * (x - x0) * (x - x1) / ((x2 - x0) * (x2 - x1));
        y.into_some()
    }
}
