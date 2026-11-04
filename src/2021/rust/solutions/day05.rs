use aoc_lib::cartesian::{Grid, Index, Size};
use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution05;
impl Solution05 {
    fn parse(input: ProblemInput) -> Vec<(Index, Index)> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (start, end) = l.split_once(" -> ").unwrap();
                let start_idx = Index::from_tuple(start.split(',').parsed().collect_tuple().unwrap());
                let end_idx = Index::from_tuple(end.split(',').parsed().collect_tuple().unwrap());
                (start_idx.min(end_idx), start_idx.max(end_idx)) // min max ?
            })
            .collect()
    }

    fn count_dangerous(vents: &[(Index, Index)], allow_diagonals: bool) -> usize {
        // Create area grid
        let width = vents.iter().map(|(start, end)| start.i.max(end.i)).max().unwrap() + 1;
        let height = vents.iter().map(|(start, end)| start.j.max(end.j)).max().unwrap() + 1;
        let mut area = Grid::empty(Size::new(width, height), 0);

        for (start, end) in vents {
            if start.i == end.i {
                // Vertical vents
                for j in start.j..=end.j {
                    *area.get_mut(&Index::new(start.i, j)) += 1;
                }
            } else if start.j == end.j {
                // Horizontal vents
                for i in start.i..=end.i {
                    *area.get_mut(&Index::new(i, start.j)) += 1;
                }
            } else if start.i.abs_diff(end.i) == start.j.abs_diff(end.j) {
                // Diagonal vents
                if allow_diagonals {
                    let j_range = start.j..=end.j;
                    let mut i_range = (start.i.min(end.i)..=start.i.max(end.i)).collect_vec();
                    if end.i < start.i {
                        i_range.reverse()
                    }

                    for (i, j) in i_range.into_iter().zip_eq(j_range) {
                        *area.get_mut(&Index { i, j }) += 1;
                    }
                }
            } else {
                panic!()
            }
        }

        area.iter().filter(|n| **n >= 2).count()
    }
}

impl Solution for Solution05 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(5),
            ProblemResult::USize(5197),
            ProblemResult::USize(12),
            ProblemResult::USize(18605),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let vents = Self::parse(input);
        let dangerous = Self::count_dangerous(&vents, false);
        dangerous.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let vents = Self::parse(input);
        let dangerous = Self::count_dangerous(&vents, true);
        dangerous.to_result()
    }
}
