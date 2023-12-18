use std::collections::HashSet;

use itertools::Itertools;

use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

type Position = (usize, usize);
pub struct Solution11;
impl Solution11 {
    fn parse(&self, input: ProblemInput) -> (usize, usize, Vec<Position>) {
        let galaxies = input
            .enumerated_grid()
            .into_iter()
            .flatten()
            .filter(|(_, _, c)| *c == '#')
            .map(|(y, x, _)| (y, x))
            .collect_vec();

        let (height, width) = input.grid_size();
        (height, width, galaxies)
    }

    fn min_distances(&self, exp_galaxies: &[Position]) -> usize {
        exp_galaxies
            .iter()
            .tuple_combinations()
            .map(|(f, s)| self.distance(f, s))
            .sum()
    }

    fn expand_galaxies(&self, height: usize, width: usize, galaxies: Vec<Position>, expansion: usize) -> Vec<Position> {
        let galaxy_rows = galaxies.iter().map(|(y, _)| *y).collect::<HashSet<_>>();
        let galaxy_cols = galaxies.iter().map(|(_, x)| *x).collect::<HashSet<_>>();
        let rows = HashSet::from_iter(0..height);
        let cols = HashSet::from_iter(0..width);

        let exp_rows = rows.difference(&galaxy_rows).sorted().collect_vec();
        let exp_cols = cols.difference(&galaxy_cols).sorted().collect_vec();

        galaxies
            .into_iter()
            .map(|(y, x)| {
                (
                    y + exp_rows.iter().take_while(|&&&ey| ey < y).count() * (expansion - 1),
                    x + exp_cols.iter().take_while(|&&&ex| ex < x).count() * (expansion - 1),
                )
            })
            .collect_vec()
    }

    fn distance(&self, first: &Position, second: &Position) -> usize {
        first.0.abs_diff(second.0) + first.1.abs_diff(second.1)
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
