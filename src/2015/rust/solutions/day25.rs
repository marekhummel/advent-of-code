use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use regex::Regex;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let (trow, tcol) = parse(input);
    let (mut row, mut col) = (1, 1);
    let mut code = 20151125u64;
    while row != trow || col != tcol {
        code = (code * 252533) % 33554393;

        row -= 1;
        col += 1;
        if row == 0 {
            row = col;
            col = 1;
        }
    }

    code.into()
}

pub fn part2(_input: ProblemInput, _is_sample: bool) -> ProblemResult {
    // No part two
    ProblemResult::NoPartTwo
}

fn parse(input: ProblemInput) -> (usize, usize) {
    let rgx = Regex::new(r"Enter the code at row (?P<row>\d+), column (?P<col>\d+).").unwrap();
    let line = input.string();
    let captures = rgx.captures(&line).unwrap();
    let row = captures.name("row").unwrap().as_str().parse().unwrap();
    let column = captures.name("col").unwrap().as_str().parse().unwrap();
    (row, column)
}
