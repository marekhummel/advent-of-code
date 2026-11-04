use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use std::collections::HashSet;

type Path = Vec<(Direction, u32)>;

pub struct Solution03;
impl Solution03 {
    fn parse(input: ProblemInput) -> (Path, Path) {
        input
            .lines()
            .into_iter()
            .map(|line| {
                line.split(',')
                    .map(|segment| {
                        let (dir, length) = segment.split_at(1);
                        (dir.try_into().unwrap(), length.parse().unwrap())
                    })
                    .collect()
            })
            .collect_tuple()
            .unwrap()
    }

    fn trace_path(path: Path) -> Vec<Position> {
        path.into_iter()
            .fold((Vec::new(), Position::zero()), |(mut trace, mut pos), (dir, length)| {
                for _ in 0..length {
                    pos = pos.advance_by(dir, 1);
                    trace.push(pos);
                }

                (trace, pos)
            })
            .0
    }
}

impl Solution for Solution03 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U128(135),
            ProblemResult::U128(2193),
            ProblemResult::USize(410),
            ProblemResult::USize(63526),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (path1, path2) = Self::parse(input);
        let trace1: HashSet<_> = HashSet::from_iter(Self::trace_path(path1));
        let trace2: HashSet<_> = HashSet::from_iter(Self::trace_path(path2));

        let intersections = trace1.intersection(&trace2);

        intersections
            .into_iter()
            .map(|pos| pos.dist(&Position::zero()))
            .min()
            .unwrap()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (path1, path2) = Self::parse(input);
        let trace1 = Self::trace_path(path1);
        let trace2 = Self::trace_path(path2);

        let trace_set1: HashSet<_> = HashSet::from_iter(trace1.iter());
        let trace_set2: HashSet<_> = HashSet::from_iter(trace2.iter());
        let intersections = trace_set1.intersection(&trace_set2);

        let closest_intersection = intersections
            .into_iter()
            .map(|&pos| trace1.iter().position(|t| t == pos).unwrap() + trace2.iter().position(|t| t == pos).unwrap())
            .min()
            .unwrap();
        (closest_intersection + 2).to_result()
    }
}
