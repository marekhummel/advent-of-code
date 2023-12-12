use std::collections::HashSet;

use itertools::Itertools;

use crate::solution::{ProblemInput, ProblemResult, Solution};

type Position = (usize, usize);
pub struct Solution11;
impl Solution11 {
    fn parse(&self, input: ProblemInput) -> (usize, usize, Vec<Position>) {
        let (height, width) = (input.len(), input[0].len());
        let galaxies = input
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch == '#')
                    .map(|(x, _)| (y, x))
                    .collect_vec()
            })
            .collect_vec();

        (height, width, galaxies)
    }

    fn min_distances(&self, exp_galaxies: &[Position]) -> usize {
        exp_galaxies
            .iter()
            .combinations(2)
            .map(|c| c.into_iter().collect_tuple().unwrap())
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
    fn get_day(&self) -> u8 {
        11
    }

    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        let (height, width, galaxies) = self.parse(input);
        let expanded_galaxies = self.expand_galaxies(height, width, galaxies, 2);
        Some(self.min_distances(&expanded_galaxies).into())
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        let (height, width, galaxies) = self.parse(input);
        let expanded_galaxies = self.expand_galaxies(height, width, galaxies, 1000000);
        Some(self.min_distances(&expanded_galaxies).into())
    }
}
