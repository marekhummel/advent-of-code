use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let (path1, path2) = parse(input);
    let trace1: HashSet<_> = HashSet::from_iter(trace_path(path1));
    let trace2: HashSet<_> = HashSet::from_iter(trace_path(path2));

    let intersections = trace1.intersection(&trace2);

    intersections
        .into_iter()
        .map(|pos| pos.dist(&Position::zero()))
        .min()
        .unwrap()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let (path1, path2) = parse(input);
    let trace1 = trace_path(path1);
    let trace2 = trace_path(path2);

    let trace_set1: HashSet<_> = HashSet::from_iter(trace1.iter());
    let trace_set2: HashSet<_> = HashSet::from_iter(trace2.iter());
    let intersections = trace_set1.intersection(&trace_set2);

    let closest_intersection = intersections
        .into_iter()
        .map(|&pos| trace1.iter().position(|t| t == pos).unwrap() + trace2.iter().position(|t| t == pos).unwrap())
        .min()
        .unwrap();
    (closest_intersection + 2).into()
}

type Path = Vec<(Direction, u32)>;

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
