use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let pairs = parse(input);
    pairs.into_iter().filter(fully_contain).count().into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let pairs = parse(input);
    pairs.into_iter().filter(overlap).count().into()
}

type Pair = ((u32, u32), (u32, u32));

fn parse(input: ProblemInput) -> Vec<Pair> {
    input
        .lines()
        .into_iter()
        .map(|l| {
            let (a0, b0, a1, b1) = l.split(&['-', ',']).parsed().collect_tuple().unwrap();
            ((a0, b0), (a1, b1))
        })
        .collect()
}

fn fully_contain(((a0, b0), (a1, b1)): &Pair) -> bool {
    (a0 <= a1 && b1 <= b0) || (a1 <= a0 && b0 <= b1)
}

fn overlap(((a0, b0), (a1, b1)): &Pair) -> bool {
    a0 <= b1 && a1 <= b0
}
