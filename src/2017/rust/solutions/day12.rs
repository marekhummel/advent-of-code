use aoc_lib::graph::Graph;
use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let pipes = parse(input);
    let components = pipes.components();

    let group0 = components.into_iter().find(|c| c.contains(&0)).unwrap();
    group0.len().into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let pipes = parse(input);
    let components = pipes.components();
    components.len().into()
}

fn parse(input: ProblemInput) -> Graph<u32> {
    input
        .lines()
        .into_iter()
        .map(|l| {
            let (src, trgs_str) = l.split_once("<->").unwrap();
            let trgs = trgs_str.split(',').parsed().collect();
            (src.trim().parse().unwrap(), trgs)
        })
        .collect()
}
