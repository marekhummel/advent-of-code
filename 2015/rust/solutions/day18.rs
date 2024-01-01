use aoc_lib::solution::Solution;
use aoc_lib::types::{Grid, IntoSome, ProblemInput, ProblemResult};
use aoc_lib::util::{Index, Size};
use itertools::Itertools;

type BoolGrid = Grid<bool>;

pub struct Solution18;
impl Solution18 {
    fn parse(input: ProblemInput) -> BoolGrid {
        input
            .grid()
            .into_iter()
            .map(|row| row.iter().map(|c| *c == '#').collect_vec())
            .collect_vec()
    }

    fn animation_step(state: BoolGrid, broken_grid: bool) -> BoolGrid {
        let size = Size::from_grid(&state);
        let mut next_state = vec![vec![false; size.width]; size.height];
        for j in 0..size.height {
            for i in 0..size.width {
                let idx = Index { i, j };
                let active_neighbors = idx
                    .moore_neighbors(size)
                    .into_iter()
                    .filter(|n| *n.grid_get(&state))
                    .count();

                if *idx.grid_get(&state) {
                    idx.grid_set(&mut next_state, active_neighbors == 2 || active_neighbors == 3);
                } else {
                    idx.grid_set(&mut next_state, active_neighbors == 3);
                }
            }
        }

        if broken_grid {
            Index { i: 0, j: 0 }.grid_set(&mut next_state, true);
            Index {
                i: 0,
                j: size.height - 1,
            }
            .grid_set(&mut next_state, true);
            Index {
                i: size.width - 1,
                j: 0,
            }
            .grid_set(&mut next_state, true);
            Index {
                i: size.width - 1,
                j: size.height - 1,
            }
            .grid_set(&mut next_state, true);
        }

        next_state
    }
}

impl Solution for Solution18 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let initial_grid = Self::parse(input);
        let steps = if _is_sample { 4 } else { 100 };
        let final_grid = (0..steps).fold(initial_grid, |state, _| Self::animation_step(state, false));

        final_grid
            .into_iter()
            .map(|row| row.iter().filter(|c| **c).count())
            .sum::<usize>()
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut initial_grid = Self::parse(input);
        let size = Size::from_grid(&initial_grid);
        Index { i: 0, j: 0 }.grid_set(&mut initial_grid, true);
        Index {
            i: 0,
            j: size.height - 1,
        }
        .grid_set(&mut initial_grid, true);
        Index {
            i: size.width - 1,
            j: 0,
        }
        .grid_set(&mut initial_grid, true);
        Index {
            i: size.width - 1,
            j: size.height - 1,
        }
        .grid_set(&mut initial_grid, true);

        let steps = if _is_sample { 5 } else { 100 };
        let final_grid = (0..steps).fold(initial_grid, |state, _| Self::animation_step(state, true));

        final_grid
            .into_iter()
            .map(|row| row.iter().filter(|c| **c).count())
            .sum::<usize>()
            .into_some()
    }
}
