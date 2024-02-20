use std::collections::HashSet;
use std::ops::Add;

use aoc_lib::cartesian::Grid;
use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos<const DIM: usize>([i32; DIM]);

impl<const DIM: usize> Pos<DIM> {
    fn stretch(coords: Vec<i32>, default: i32) -> Self {
        let mut pos = Pos([default; DIM]);
        for (i, c) in coords.into_iter().enumerate() {
            pos.0[i] = c;
        }
        pos
    }

    fn neighbors(&self) -> Vec<Pos<DIM>> {
        let deltas = [-1, 0, 1];
        (0..DIM)
            .map(|_| deltas)
            .multi_cartesian_product()
            .filter(|ds| !ds.iter().all(|d| *d == 0))
            .map(|ds| *self + ds)
            .collect()
    }
}

impl<const DIM: usize> Add<Vec<i32>> for Pos<DIM> {
    type Output = Self;

    fn add(self, rhs: Vec<i32>) -> Self::Output {
        let mut res = self;
        for i in 0..DIM {
            res.0[i] += rhs[i];
        }
        res
    }
}

pub struct Solution17;
impl Solution17 {
    fn cycle<const N: usize>(
        state: HashSet<Pos<N>>,
        min_pos: Pos<N>,
        max_pos: Pos<N>,
    ) -> (HashSet<Pos<N>>, Pos<N>, Pos<N>) {
        let mut new_state = HashSet::new();

        for coords in (0..N).map(|i| min_pos.0[i]..=max_pos.0[i]).multi_cartesian_product() {
            let pos = Pos(coords.try_into().unwrap());
            let active_neighbors = pos.neighbors().into_iter().filter(|nb| state.contains(nb)).count();
            if active_neighbors == 3 || (state.contains(&pos) && active_neighbors == 2) {
                new_state.insert(pos);
            }
        }

        let (mins, maxs): (Vec<_>, Vec<_>) = (0..N)
            .map(|i| {
                new_state
                    .iter()
                    .flat_map(|p| [p.0[i] - 1, p.0[i] + 1]) // to extend range in each direction
                    .minmax()
                    .into_option()
                    .unwrap()
            })
            .unzip();

        let new_min_pos = Pos(mins.try_into().unwrap());
        let new_max_pos = Pos(maxs.try_into().unwrap());
        (new_state, new_min_pos, new_max_pos)
    }

    fn initial_state<const N: usize>(grid: Grid<char>) -> (HashSet<Pos<N>>, Pos<N>, Pos<N>) {
        let state = grid
            .enumerate()
            .filter(|(_, c)| **c == '#')
            .map(|(idx, _)| Pos::stretch(vec![idx.i as i32, idx.j as i32], 0))
            .collect::<HashSet<_>>();
        let min_pos = Pos([-1; N]);
        let max_pos = Pos::stretch(vec![grid.size.width as i32, grid.size.height as i32], 1);

        (state, min_pos, max_pos)
    }

    fn boot_process<const N: usize>(mut state: HashSet<Pos<N>>, mut min_pos: Pos<N>, mut max_pos: Pos<N>) -> usize {
        for _ in 0..6 {
            (state, min_pos, max_pos) = Self::cycle(state, min_pos, max_pos);
        }
        state.len()
    }

    #[allow(dead_code)]
    fn print_state(state: &HashSet<Pos<3>>, min_pos: &Pos<3>, max_pos: &Pos<3>) {
        for z in min_pos.0[2]..=max_pos.0[2] {
            println!("z={z}");
            for y in min_pos.0[1]..=max_pos.0[1] {
                for x in min_pos.0[0]..=max_pos.0[0] {
                    print!("{}", if state.contains(&Pos([x, y, z])) { "#" } else { "." });
                }
                println!()
            }
            println!()
        }
    }
}

impl Solution for Solution17 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let grid = input.grid();
        let (state, min_pos, max_pos) = Self::initial_state::<3>(grid);
        Self::boot_process(state, min_pos, max_pos).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let grid = input.grid();
        let (state, min_pos, max_pos) = Self::initial_state::<4>(grid);
        Self::boot_process(state, min_pos, max_pos).to_result()
    }
}
