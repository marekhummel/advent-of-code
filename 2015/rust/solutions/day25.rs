use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use regex::Regex;

pub struct Solution25;
impl Solution25 {
    fn parse(input: ProblemInput) -> (usize, usize) {
        let rgx = Regex::new(r"To continue, please consult the code grid in the manual.  Enter the code at row (?P<row>\d+), column (?P<col>\d+).").unwrap();
        let line = input.string();
        let captures = rgx.captures(&line).unwrap();
        let row = captures.name("row").unwrap().as_str().parse().unwrap();
        let column = captures.name("col").unwrap().as_str().parse().unwrap();
        (row, column)
    }
}

impl Solution for Solution25 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let (trow, tcol) = Self::parse(input);
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

        code.into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        // No part two
        None
    }
}
