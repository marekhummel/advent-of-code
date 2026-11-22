use std::collections::HashSet;

use aoc_lib::cartesian::{Index, Size};
use itertools::Itertools;

use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let (galaxies, size) = parse(input);
    let expanded_galaxies = expand_galaxies(size, galaxies, 2);
    min_distances(&expanded_galaxies).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let (galaxies, size) = parse(input);
    let expanded_galaxies = expand_galaxies(size, galaxies, 1000000);
    min_distances(&expanded_galaxies).into()
}

fn parse(input: ProblemInput) -> (Vec<Index>, Size) {
    let grid = input.grid();
    let galaxies = grid
        .enumerate()
        .filter(|(_, c)| **c == '#')
        .map(|(idx, _)| idx)
        .collect_vec();

    (galaxies, grid.size)
}

fn min_distances(exp_galaxies: &[Index]) -> usize {
    exp_galaxies
        .iter()
        .array_combinations()
        .map(|[f, s]| distance(f, s))
        .sum()
}

fn expand_galaxies(size: Size, galaxies: Vec<Index>, expansion: usize) -> Vec<Index> {
    let galaxy_rows = galaxies.iter().map(|idx| idx.j).collect::<HashSet<_>>();
    let galaxy_cols = galaxies.iter().map(|idx| idx.i).collect::<HashSet<_>>();
    let rows = HashSet::from_iter(0..size.height);
    let cols = HashSet::from_iter(0..size.width);

    let exp_rows = rows.difference(&galaxy_rows).sorted().collect_vec();
    let exp_cols = cols.difference(&galaxy_cols).sorted().collect_vec();

    galaxies
        .into_iter()
        .map(|idx| {
            Index::new(
                idx.i + exp_cols.iter().take_while(|&&&ei| ei < idx.i).count() * (expansion - 1),
                idx.j + exp_rows.iter().take_while(|&&&ej| ej < idx.j).count() * (expansion - 1),
            )
        })
        .collect_vec()
}

fn distance(first: &Index, second: &Index) -> usize {
    first.i.abs_diff(second.i) + first.j.abs_diff(second.j)
}
