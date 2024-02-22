use aoc_lib::cartesian::Grid;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution18;
impl Solution18 {
    fn parse(input: ProblemInput) -> Grid<bool> {
        input.grid().map_elements(|c| *c == '#')
    }

    fn animation_step(state: Grid<bool>, broken_grid: bool) -> Grid<bool> {
        let size = state.size;
        let mut next_state = Grid::empty(size, false);
        for (idx, value) in state.enumerate() {
            let active_neighbors = idx.moore_neighbors(size).into_iter().filter(|n| *state.get(n)).count();
            if *value {
                next_state.set(&idx, active_neighbors == 2 || active_neighbors == 3);
            } else {
                next_state.set(&idx, active_neighbors == 3);
            }
        }

        if broken_grid {
            for corner in next_state.corners() {
                next_state.set(&corner, true);
            }
        }

        next_state
    }
}

impl Solution for Solution18 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(4),
            ProblemResult::USize(768),
            ProblemResult::USize(17),
            ProblemResult::USize(781),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let initial_grid = Self::parse(input);
        let steps = if is_sample { 4 } else { 100 };
        let final_grid = (0..steps).fold(initial_grid, |state, _| Self::animation_step(state, false));

        final_grid.iter().filter(|c| **c).count().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let mut initial_grid = Self::parse(input);
        for corner in initial_grid.corners() {
            initial_grid.set(&corner, true);
        }

        let steps = if is_sample { 5 } else { 100 };
        let final_grid = (0..steps).fold(initial_grid, |state, _| Self::animation_step(state, true));

        final_grid.iter().filter(|c| **c).count().to_result()
    }
}
