use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let layers = parse(input);

    layers
        .into_iter()
        .filter(|(depth, range)| depth % ((range - 1) * 2) == 0)
        .map(|(depth, range)| depth * range)
        .sum::<usize>()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let layers = parse(input)
        .into_iter()
        .map(|(depth, range)| (depth, (range - 1) * 2))
        .collect_vec();

    (0..)
        .find(|delay| layers.iter().all(|(depth, period)| (*depth + delay) % period != 0))
        .unwrap()
        .into()
}

fn parse(input: ProblemInput) -> Vec<(usize, usize)> {
    input
        .lines()
        .into_iter()
        .map(|l| {
            let (d, r) = l.split_once(": ").unwrap();
            (d.parse().unwrap(), r.parse().unwrap())
        })
        .collect()
}
