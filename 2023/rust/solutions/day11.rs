use std::collections::HashSet;

use aoc_lib::util::Index;
use itertools::Itertools;

use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution11;
impl Solution11 {
    fn parse(&self, input: ProblemInput) -> (usize, usize, Vec<Index>) {
        let galaxies = input
            .enumerated_grid()
            .into_iter()
            .flatten()
            .filter(|(_, _, c)| *c == '#')
            .map(|(j, i, _)| Index { i, j })
            .collect_vec();

        let (height, width) = input.grid_size();
        (height, width, galaxies)
    }

    fn min_distances(&self, exp_galaxies: &[Index]) -> usize {
        exp_galaxies
            .iter()
            .tuple_combinations()
            .map(|(f, s)| self.distance(f, s))
            .sum()
    }

    fn expand_galaxies(&self, height: usize, width: usize, galaxies: Vec<Index>, expansion: usize) -> Vec<Index> {
        let galaxy_rows = galaxies.iter().map(|idx| idx.j).collect::<HashSet<_>>();
        let galaxy_cols = galaxies.iter().map(|idx| idx.i).collect::<HashSet<_>>();
        let rows = HashSet::from_iter(0..height);
        let cols = HashSet::from_iter(0..width);

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
    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        let (height, width, galaxies) = self.parse(input);
        let expanded_galaxies = self.expand_galaxies(height, width, galaxies, 2);
        self.min_distances(&expanded_galaxies).into_some()
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        let (height, width, galaxies) = self.parse(input);
        let expanded_galaxies = self.expand_galaxies(height, width, galaxies, 1000000);
        self.min_distances(&expanded_galaxies).into_some()
    }
}
