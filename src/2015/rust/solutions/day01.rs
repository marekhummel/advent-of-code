use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input
        .string()
        .chars()
        .map(|c| -(((c as u8 - 40) * 2) as i16 - 1))
        .sum::<i16>()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let index = input
        .string()
        .chars()
        .map(|c| -(((c as u8 - 40) * 2) as i16 - 1))
        .scan(0, |floor, dir| {
            *floor += dir;
            Some(*floor)
        })
        .take_while(|floor| *floor != -1)
        .count();

    (index + 1).into()
}
