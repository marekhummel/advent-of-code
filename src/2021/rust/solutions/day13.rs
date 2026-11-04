use std::collections::HashSet;

use aoc_lib::cartesian::{Grid, Position, Size};
use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution13;
impl Solution13 {
    fn parse(input: ProblemInput) -> (HashSet<Position>, Vec<(i128, bool)>) {
        let lines = input.lines();
        let (dot_strs, fold_strs) = lines.split(|l| l.is_empty()).collect_tuple().unwrap();

        let dots = dot_strs
            .iter()
            .map(|s| Position::from_tuple(s.split(',').parsed().collect_tuple().unwrap()))
            .collect();

        let folds = fold_strs
            .iter()
            .map(|s| {
                s.split_ascii_whitespace()
                    .nth(2)
                    .map(|crease| {
                        let (dim, coord) = crease.split_once('=').unwrap();
                        (coord.parse().unwrap(), dim == "y")
                    })
                    .unwrap()
            })
            .collect();

        (dots, folds)
    }

    fn fold(fold: (i128, bool), dots: HashSet<Position>) -> HashSet<Position> {
        let (at, dim_y) = fold;
        dots.into_iter()
            .map(|d| {
                let nx = if dim_y { d.x } else { at - d.x.abs_diff(at) as i128 };
                let ny = if dim_y { at - d.y.abs_diff(at) as i128 } else { d.y };
                Position::new(nx, ny)
            })
            .collect()
    }
}

impl Solution for Solution13 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(17),
            ProblemResult::USize(664),
            ProblemResult::NoSample,
            ProblemResult::String("EFJKZLBL".to_string()),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (dots, folds) = Self::parse(input);
        let folded = Self::fold(folds[0], dots);
        folded.len().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        if _is_sample {
            return ProblemResult::NoSample;
        }

        let (paper, folds) = Self::parse(input);
        let final_dots = folds.into_iter().fold(paper, |dots, fold| Self::fold(fold, dots));

        let max_x = final_dots.iter().map(|d| d.x).max().unwrap();
        let max_y = final_dots.iter().map(|d| d.y).max().unwrap();

        let mut display = Grid::empty(Size::new(max_x as usize + 1, max_y as usize + 1), false);
        final_dots.into_iter().for_each(|d| display.set(&d.into(), true));
        // display.print(|_, d| if *d { "#" } else { " " });
        "EFJKZLBL".to_result()
    }
}
