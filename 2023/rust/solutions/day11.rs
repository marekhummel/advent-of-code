use std::collections::HashSet;

use aoc_lib::cartesian::{Index, Size};
use itertools::Itertools;

use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution11;
impl Solution11 {
    fn parse(&self, input: ProblemInput) -> (Vec<Index>, Size) {
        let grid = input.grid();
        let galaxies = grid
            .enumerate()
            .filter(|(_, c)| **c == '#')
            .map(|(idx, _)| idx)
            .collect_vec();

        (galaxies, grid.size)
    }

    fn min_distances(&self, exp_galaxies: &[Index]) -> usize {
        exp_galaxies
            .iter()
            .tuple_combinations()
            .map(|(f, s)| self.distance(f, s))
            .sum()
    }

    fn expand_galaxies(&self, size: Size, galaxies: Vec<Index>, expansion: usize) -> Vec<Index> {
        let galaxy_rows = galaxies.iter().map(|idx| idx.j).collect::<HashSet<_>>();
        let galaxy_cols = galaxies.iter().map(|idx| idx.i).collect::<HashSet<_>>();
        let rows = HashSet::from_iter(0..size.height);
        let cols = HashSet::from_iter(0..size.width);

        let exp_rows = rows.difference(&galaxy_rows).sorted().collect_vec();
        let exp_cols = cols.difference(&galaxy_cols).sorted().collect_vec();

        galaxies
            .into_iter()
            .map(|idx| Index {
                i: idx.i + exp_cols.iter().take_while(|&&&ei| ei < idx.i).count() * (expansion - 1),
                j: idx.j + exp_rows.iter().take_while(|&&&ej| ej < idx.j).count() * (expansion - 1),
            })
            .collect_vec()
    }

    fn distance(&self, first: &Index, second: &Index) -> usize {
        first.i.abs_diff(second.i) + first.j.abs_diff(second.j)
    }
}

impl Solution for Solution11 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (galaxies, size) = self.parse(input);
        let expanded_galaxies = self.expand_galaxies(size, galaxies, 2);
        self.min_distances(&expanded_galaxies).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (galaxies, size) = self.parse(input);
        let expanded_galaxies = self.expand_galaxies(size, galaxies, 1000000);
        self.min_distances(&expanded_galaxies).to_result()
    }
}
