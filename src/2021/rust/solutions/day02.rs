use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut horizontal = 0;
    let mut depth = 0;

    for line in input.lines() {
        match line.split_ascii_whitespace().collect_tuple().unwrap() {
            ("forward", x) => horizontal += x.parse::<u32>().unwrap(),
            ("down", x) => depth += x.parse::<u32>().unwrap(),
            ("up", x) => depth -= x.parse::<u32>().unwrap(),
            _ => unreachable!(),
        }
    }

    (horizontal * depth).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.lines() {
        match line.split_ascii_whitespace().collect_tuple().unwrap() {
            ("forward", x) => {
                let units = x.parse::<u32>().unwrap();
                horizontal += units;
                depth += aim * units;
            }
            ("down", x) => aim += x.parse::<u32>().unwrap(),
            ("up", x) => aim -= x.parse::<u32>().unwrap(),
            _ => unreachable!(),
        }
    }

    (horizontal * depth).into()
}
