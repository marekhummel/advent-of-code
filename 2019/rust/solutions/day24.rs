use core::panic;
use std::collections::{HashMap, HashSet};

use aoc_lib::cartesian::{Grid, Index, Size};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution24;
impl Solution24 {
    fn parse(input: ProblemInput) -> Grid<bool> {
        input.grid().map_elements(|c| *c == '#')
    }

    fn next(state: Grid<bool>) -> Grid<bool> {
        let mut new_grid = Grid::empty(state.size, false);
        for j in 0..state.size.height {
            for i in 0..state.size.width {
                let idx = Index { i, j };
                let adjacent = idx
                    .von_neumann_neighbors(state.size)
                    .into_iter()
                    .filter(|nb| *state.get(nb))
                    .count();

                if adjacent == 1 || (!*state.get(&idx) && adjacent == 2) {
                    new_grid.set(&idx, true);
                }
            }
        }

        new_grid
    }

    fn biodiversity(state: &Grid<bool>) -> u32 {
        state.iter().rev().fold(0u32, |rating, bug| (rating << 1) + *bug as u32)
    }

    fn next_recursive(states: HashMap<i32, Grid<bool>>) -> HashMap<i32, Grid<bool>> {
        let mut new_states = HashMap::new();

        // Compute each layer
        let size = Size { width: 5, height: 5 };
        let Some((min, max)) = states.keys().minmax().into_option() else {
            unreachable!()
        };
        for level in min - 1..=max + 1 {
            let mut new_grid = Grid::empty(size, false);
            for j in 0..size.height {
                for i in 0..size.width {
                    if i == 2 && j == 2 {
                        continue;
                    }

                    let idx = Index { i, j };
                    let adjacent = Self::neighbors_recursive(level, idx, size)
                        .into_iter()
                        .filter(|(nl, nb)| *states.get(nl).map(|state| state.get(nb)).unwrap_or(&false))
                        .count();

                    let bug = *states.get(&level).map(|state| state.get(&idx)).unwrap_or(&false);
                    if adjacent == 1 || (!bug && adjacent == 2) {
                        new_grid.set(&idx, true);
                    }
                }
            }

            new_states.insert(level, new_grid);
        }

        // Remove outer layers if empty to save computations
        for outer in [min - 1, max + 1] {
            if new_states[&outer].iter().all(|bug| !*bug) {
                new_states.remove(&outer);
            }
        }

        new_states
    }

    fn neighbors_recursive(level: i32, idx: Index, size: Size) -> Vec<(i32, Index)> {
        // Start with basic Von Neumann neighborhood
        let neumann = idx
            .von_neumann_neighbors(size)
            .into_iter()
            .filter(|idx| *idx != Index { i: 2, j: 2 })
            .map(|idx| (level, idx))
            .collect_vec();

        // Add neighbors from other layers
        let other_levels = match idx {
            Index { i: 2, j: 1 } => (0..5).map(|i| (level + 1, Index { i, j: 0 })).collect(),
            Index { i: 1, j: 2 } => (0..5).map(|j| (level + 1, Index { i: 0, j })).collect(),
            Index { i: 3, j: 2 } => (0..5).map(|j| (level + 1, Index { i: 4, j })).collect(),
            Index { i: 2, j: 3 } => (0..5).map(|i| (level + 1, Index { i, j: 4 })).collect(),
            Index { i: 2, j: 2 } => panic!("Recursion hole?"),
            _ => {
                let mut extra = Vec::new();
                if idx.i == 0 {
                    extra.push((level - 1, Index { i: 1, j: 2 }))
                }
                if idx.i == 4 {
                    extra.push((level - 1, Index { i: 3, j: 2 }))
                }
                if idx.j == 0 {
                    extra.push((level - 1, Index { i: 2, j: 1 }))
                }
                if idx.j == 4 {
                    extra.push((level - 1, Index { i: 2, j: 3 }))
                }
                extra
            }
        };

        [neumann, other_levels].concat()
    }
}

impl Solution for Solution24 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut state = Self::parse(input);

        // Use biodiversity instead of grid in hashset, as the rating is essentially a integer representation of it
        let mut seen = HashSet::new();
        loop {
            let rating = Self::biodiversity(&state);
            if seen.contains(&rating) {
                return rating.to_result();
            }
            seen.insert(rating);

            state = Self::next(state);
        }
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let initial_states = HashMap::from([(0, Self::parse(input))]);
        let minutes = if _is_sample { 10 } else { 200 };

        let final_states = (0..minutes).fold(initial_states, |states, _| Self::next_recursive(states));
        final_states
            .values()
            .map(|state| state.iter().filter(|bug| **bug).count())
            .sum::<usize>()
            .to_result()
    }
}
