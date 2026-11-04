use std::collections::HashMap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution25;
impl Solution25 {}

impl Solution for Solution25 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::String("2=-1=0".to_string()),
            ProblemResult::String("2=--=0000-1-0-=1=0=2".to_string()),
            ProblemResult::NoPartTwo,
            ProblemResult::NoPartTwo,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let lookup: HashMap<char, i8> = "=-012".chars().zip_eq(-2..=2).collect();
        let snafus = input.lines().into_iter();
        let fuels_reqs = snafus.map(|snafu| snafu.chars().map(|c| lookup[&c]).fold(0i64, |n, d| n * 5 + d as i64));
        let mut total_fuel: i64 = fuels_reqs.sum();

        // Convert to snafu
        let rev_lookup: HashMap<_, _> = lookup.into_iter().map(|(c, d)| (d, c)).collect();
        let mut console = vec![];
        let mut carry = false;
        while total_fuel > 0 {
            let digit = (total_fuel % 5) as i8 + (carry as i8);
            carry = digit > 2;
            total_fuel /= 5;
            console.push(rev_lookup[&if carry { digit - 5 } else { digit }]);
        }

        if carry {
            console.push('1')
        }

        console.into_iter().rev().collect::<String>().to_result()
    }

    fn solve_version02(&self, _input: ProblemInput, _is_sample: bool) -> ProblemResult {
        ProblemResult::NoPartTwo
    }
}
