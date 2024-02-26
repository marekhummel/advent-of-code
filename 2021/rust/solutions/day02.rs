use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution02;
impl Solution02 {}

impl Solution for Solution02 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(150),
            ProblemResult::U32(1840243),
            ProblemResult::U32(900),
            ProblemResult::U32(1727785422),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
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

        (horizontal * depth).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
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

        (horizontal * depth).to_result()
    }
}
