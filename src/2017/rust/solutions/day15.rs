use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let (ga, gb) = parse(input);

    let total = ga
        .zip(gb)
        .take(40_000_000)
        .filter(|(a, b)| a & 0xffff == b & 0xffff)
        .count();

    total.into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let (ga, gb) = parse(input);

    let picky_ga = ga.filter(|a| a % 4 == 0);
    let picky_gb = gb.filter(|a| a % 8 == 0);

    let total = picky_ga
        .zip(picky_gb)
        .take(5_000_000)
        .filter(|(a, b)| a & 0xffff == b & 0xffff)
        .count();

    total.into()
}

struct Generator {
    last: u64,
    factor: u64,
}

impl Iterator for Generator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let n = (self.last * self.factor) % ((1 << 31) - 1);
        self.last = n;
        Some(n as u32)
    }
}

fn parse(input: ProblemInput) -> (Generator, Generator) {
    input
        .lines()
        .into_iter()
        .enumerate()
        .map(|(i, l)| {
            let factor = if i == 0 { 16807 } else { 48271 };
            let last = l.split_whitespace().last().unwrap().parse().unwrap();
            Generator { last, factor }
        })
        .collect_tuple()
        .unwrap()
}
