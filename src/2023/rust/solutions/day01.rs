use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input.lines().iter().map(|s| value01(s)).sum::<u32>().into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input.lines().iter().map(|s| value02(s)).sum::<u32>().into()
}

const DIGITS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5", "6",
    "7", "8", "9",
];

fn value01(s: &str) -> u32 {
    let first = s.chars().find(char::is_ascii_digit).unwrap();
    let last = s.chars().rev().find(char::is_ascii_digit).unwrap();
    format!("{first}{last}").parse().unwrap()
}

fn value02(s: &str) -> u32 {
    let first = DIGITS
        .iter()
        .map(|d| s.find(d))
        .enumerate()
        .filter(|(_, p)| p.is_some())
        .min_by_key(|(_, p)| p.unwrap())
        .unwrap()
        .0;

    let last = DIGITS
        .iter()
        .map(|d| s.rfind(d))
        .enumerate()
        .filter(|(_, p)| p.is_some())
        .max_by_key(|(_, p)| p.unwrap())
        .unwrap()
        .0;

    ((first % 10) * 10 + (last % 10)) as u32
}
