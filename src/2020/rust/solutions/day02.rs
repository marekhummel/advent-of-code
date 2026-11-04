use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution02;
impl Solution02 {}

impl Solution for Solution02 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
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
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
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
            .to_result()
    }
}
