use std::collections::HashMap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution08;
impl Solution08 {
    fn parse(input: ProblemInput) -> Vec<(Vec<String>, Vec<String>)> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (wire_str, num_str) = l.split_once(" | ").unwrap();
                let wires = wire_str.split_ascii_whitespace().map(|s| s.to_string()).collect();
                let number = num_str.split_ascii_whitespace().map(|s| s.to_string()).collect();
                (wires, number)
            })
            .collect()
    }
}

impl Solution for Solution08 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(26),
            ProblemResult::USize(470),
            ProblemResult::USize(61229),
            ProblemResult::USize(989396),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let entries = Self::parse(input);

        entries
            .into_iter()
            .flat_map(|(_, num)| num)
            .filter(|digit| [2, 3, 4, 7].contains(&digit.len()))
            .count()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let entries = Self::parse(input);
        let displays = [
            "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
        ];

        let mut output = 0;
        for (signal_patterns, number) in entries {
            // To speed up, instead of trying every permutation we could make some assumptions
            // E.g. deduce the mapping of 'a' by comparing the signals for 1 and 7 and so forth.
            for permutaion in "abcdefg".chars().permutations(7) {
                // Create mapping
                let mapping: HashMap<_, _> = "abcdefg".chars().zip_eq(permutaion.into_iter()).collect();

                // Check if each recorded signal pattern leads to a valid digit
                let mut valid = true;
                for pattern in &signal_patterns {
                    let segments: String = pattern.chars().map(|c| mapping[&c]).sorted().collect();
                    if !displays.contains(&segments.as_str()) {
                        valid = false;
                        break;
                    }
                }

                // If mapping is valid, apply to number
                if valid {
                    let value = number
                        .iter()
                        .map(|digit| {
                            let displayed = digit.chars().map(|c| mapping[&c]).sorted().collect::<String>();
                            let digit_value = displays.iter().position(|disp| disp == &displayed.as_str()).unwrap();
                            digit_value
                        })
                        .fold(0, |val, digit| val * 10 + digit);
                    output += value;
                    break;
                }
            }
        }

        output.to_result()
    }
}
