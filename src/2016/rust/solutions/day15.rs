use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;
use regex::Regex;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let discs = parse(input);
    find_time_naive(&discs).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut discs = parse(input);
    discs.push((discs.len() as u32 + 1, 11, 0));
    find_time_naive(&discs).into()
}

type Disc = (u32, u32, u32);

fn parse(input: ProblemInput) -> Vec<Disc> {
    let rgx = Regex::new(r"^Disc #(?P<n>\d+) has (?P<p>\d+) positions; at time=0, it is at position (?P<s>\d+).$")
        .unwrap();
    input
        .lines()
        .into_iter()
        .map(|l| {
            let captures = rgx.captures(&l).unwrap();
            let n = captures.name("n").unwrap().as_str().parse().unwrap();
            let p = captures.name("p").unwrap().as_str().parse().unwrap();
            let s = captures.name("s").unwrap().as_str().parse().unwrap();
            (n, p, s)
        })
        .collect_vec()
}

// Could also be solved with the Chinese Remainder Theorem, since all p are pairwise coprime.
// However the search space is limited by the product of all p, which is ~4M and thus viable for brute force.
fn find_time_naive(discs: &[Disc]) -> u32 {
    for t in 0.. {
        let mut disc_pos = discs.iter().map(|(n, p, s)| (t + n + s) % p);
        if disc_pos.all(|dp| dp == 0) {
            return t;
        }
    }

    unreachable!()
}
