use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

const DIGITS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5", "6",
    "7", "8", "9",
];
pub struct Solution01;

impl Solution01 {
    fn value01(&self, s: &str) -> u32 {
        let first = s.chars().find(char::is_ascii_digit).unwrap();
        let last = s.chars().rev().find(char::is_ascii_digit).unwrap();
        format!("{first}{last}").parse().unwrap()
    }

    fn value02(&self, s: &str) -> u32 {
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
}

impl Solution for Solution01 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(142),
            ProblemResult::U32(55971),
            ProblemResult::U32(281),
            ProblemResult::U32(54719),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        input.lines().iter().map(|s| self.value01(s)).sum::<u32>().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        input.lines().iter().map(|s| self.value02(s)).sum::<u32>().to_result()
    }
}
