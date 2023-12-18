use itertools::Itertools;

use aoc_lib::solution::Solution;
use aoc_lib::types::{Grid, IntoSome, ProblemInput, ProblemResult};

type BoolGrid = Grid<bool>;
pub struct Solution13;
impl Solution13 {
    fn parse(&self, input: ProblemInput) -> Vec<BoolGrid> {
        input
            .lines()
            .into_iter()
            .map(|s| s.chars().map(|c| c == '#').collect_vec())
            .collect_vec()
            .split(|row| row.is_empty())
            .map(|row| row.to_vec())
            .collect_vec()
    }

    fn compute_reflection_value(&self, grid: &BoolGrid, smudges: usize) -> usize {
        match self.find_reflection(grid, smudges) {
            Some(mirror_hor) => 100 * (mirror_hor + 1),
            None => {
                let grid_t = self.transpose(grid);
                match self.find_reflection(&grid_t, smudges) {
                    Some(mirror_ver) => mirror_ver + 1,
                    None => unreachable!("No mirror found"),
                }
            }
        }
    }

    fn transpose(&self, grid: &BoolGrid) -> BoolGrid {
        (0..grid[0].len())
            .map(|i| grid.iter().map(|inner| inner[i]).collect_vec())
            .collect()
    }

    fn find_reflection(&self, grid: &BoolGrid, total_smudges: usize) -> Option<usize> {
        // Find possible horizontal mirrors (between two almost equal lines)
        // Result is a list of possible mirrors, where each entry is the mirrors position plus
        // the list of rows that match and if that match includes a smudge or not
        let mirrors = grid
            .iter()
            .enumerate()
            .tuple_combinations()
            .filter(|((i, _), (j, _))| (i + j) & 1 == 1) // mirrors can only be between rows
            .map(|((i, r), (j, s))| ((i + j) / 2, i, j, Self::count_diff(r, s))) // find mirror position and amount of smudges needed for equality
            .filter(|(_, _, _, d)| *d <= total_smudges) // filter out pairs that need more than one smudge
            .sorted_by_key(|(m, ..)| *m) // sort for grouping
            .group_by(|(m, ..)| *m) // group by mirror position
            .into_iter()
            .map(|(key, group)| (key, group.map(|(_, i, j, d)| (i, j, d == 1)).collect_vec()))
            .collect_vec();

        // Find first mirror which actually mirrors entire grid with expected amount of smudges
        mirrors
            .into_iter()
            .find(|(mirror, row_pairs)| {
                let mut smudges = 0;
                for offset in Self::offset_range(*mirror, grid.len()) {
                    let (low, high) = (mirror - offset, mirror + 1 + offset);
                    match row_pairs.iter().find(|&(i, j, _)| *i == low && *j == high) {
                        Some((_, _, false)) => (),
                        Some((_, _, true)) if smudges >= total_smudges => return false,
                        Some((_, _, true)) => smudges += 1,
                        None => return false,
                    }
                }
                smudges == total_smudges // return only true if the required number of smudges has been fixed
            })
            .map(|(m, _)| m)
    }

    fn count_diff<T: PartialEq>(first: &[T], second: &[T]) -> usize {
        first.iter().zip_eq(second.iter()).filter(|(v, w)| v != w).count()
    }

    fn offset_range(mirror_pos: usize, length: usize) -> impl IntoIterator<Item = usize> {
        0..=(usize::min(length - mirror_pos - 2, mirror_pos))
    }
}

impl Solution for Solution13 {
    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        self.parse(input)
            .into_iter()
            .map(|grid| self.compute_reflection_value(&grid, 0))
            .sum::<usize>()
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        self.parse(input)
            .into_iter()
            .map(|grid| self.compute_reflection_value(&grid, 1))
            .sum::<usize>()
            .into_some()
    }
}
