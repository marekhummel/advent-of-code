use aoc_lib::cartesian::{HexDirection, HexIndex};
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let path = parse(input);

    let start = HexIndex { x: 0, y: 0 };
    let mut target = start.clone();
    for dir in path {
        target = target.step(&dir);
    }

    start.dist(&target).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let path = parse(input);

    let start = HexIndex { x: 0, y: 0 };
    let mut target = start.clone();
    let mut max_dist = 0;
    for dir in path {
        target = target.step(&dir);
        max_dist = max_dist.max(start.dist(&target))
    }

    max_dist.into()
}

fn parse(input: ProblemInput) -> Vec<HexDirection> {
    input.string().split(',').flat_map(|d| d.try_into()).collect_vec()
}
