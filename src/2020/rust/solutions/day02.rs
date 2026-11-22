use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input
        .lines()
        .into_iter()
        .filter(|line| {
            let (policy, pwd) = line.split_once(": ").unwrap();
            let (range, letter) = policy.split_once(' ').unwrap();
            let (low, high) = range.split('-').parsed().collect_tuple().unwrap();

            let occurances = pwd.trim().chars().filter(|l| l.to_string() == letter).count();
            low <= occurances && occurances <= high
        })
        .count()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input
        .lines()
        .into_iter()
        .filter(|line| {
            let (policy, pwd) = line.split_once(": ").unwrap();
            let (range, letter) = policy.split_once(' ').unwrap();
            let (p1, p2) = range.split('-').parsed().collect_tuple().unwrap();

            (&pwd[p1 - 1..p1] == letter) ^ (&pwd[p2 - 1..p2] == letter)
        })
        .count()
        .into()
}
