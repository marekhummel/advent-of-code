use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

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

pub struct Solution15;
impl Solution15 {
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
}

impl Solution for Solution15 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (ga, gb) = Self::parse(input);

        let total = ga
            .zip(gb)
            .take(40_000_000)
            .filter(|(a, b)| a & 0xffff == b & 0xffff)
            .count();

        total.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (ga, gb) = Self::parse(input);

        let picky_ga = ga.filter(|a| a % 4 == 0);
        let picky_gb = gb.filter(|a| a % 8 == 0);

        let total = picky_ga
            .zip(picky_gb)
            .take(5_000_000)
            .filter(|(a, b)| a & 0xffff == b & 0xffff)
            .count();

        total.to_result()
    }
}
