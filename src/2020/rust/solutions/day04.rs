use std::collections::HashMap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution04;
impl Solution04 {
    fn parse(input: ProblemInput) -> Vec<HashMap<String, String>> {
        input
            .lines()
            .split(|l| l.is_empty())
            .map(|batch| {
                batch
                    .join(" ")
                    .split_whitespace()
                    .map(|prop| prop.split(':').map(|s| s.to_string()).collect_tuple().unwrap())
                    .collect()
            })
            .collect()
    }
}

impl Solution for Solution04 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; // "cid"
        let passports = Self::parse(input);

        passports
            .into_iter()
            .filter(|pp| required.iter().all(|req| pp.contains_key(&req.to_string())))
            .count()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let passports = Self::parse(input);
        let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; // "cid"
        let eye_colours = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let hex_digits = "0123456789abcdef".chars().collect_vec();

        #[allow(clippy::manual_range_contains)]
        passports
            .into_iter()
            .filter(|pp| required.iter().all(|req| pp.contains_key(&req.to_string())))
            .filter(|pp| {
                pp.iter().all(|(field, value)| match field.as_str() {
                    "byr" => value.parse::<u16>().map(|yr| 1920 <= yr && yr <= 2002).unwrap_or(false),
                    "iyr" => value.parse::<u16>().map(|yr| 2010 <= yr && yr <= 2020).unwrap_or(false),
                    "eyr" => value.parse::<u16>().map(|yr| 2020 <= yr && yr <= 2030).unwrap_or(false),
                    "hgt" if value.ends_with("cm") => value
                        .trim_end_matches("cm")
                        .parse::<u16>()
                        .map(|h| 150 <= h && h <= 193)
                        .unwrap_or(false),
                    "hgt" if value.ends_with("in") => value
                        .trim_end_matches("in")
                        .parse::<u16>()
                        .map(|h| 59 <= h && h <= 76)
                        .unwrap_or(false),
                    "hgt" => false,
                    "hcl" => {
                        value.len() == 7
                            && value.starts_with('#')
                            && value.chars().skip(1).all(|d| hex_digits.contains(&d))
                    }
                    "ecl" => eye_colours.contains(&value.as_str()),
                    "pid" => value.len() == 9 && value.parse::<u32>().is_ok(),
                    _ => true,
                })
            })
            .count()
            .to_result()
    }
}
