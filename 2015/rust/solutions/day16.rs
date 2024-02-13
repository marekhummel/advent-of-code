use std::collections::HashMap;

use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution16;
impl Solution16 {
    fn parse(input: ProblemInput) -> Vec<HashMap<String, u8>> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                l.split_once(':')
                    .unwrap()
                    .1
                    .split(',')
                    .map(|prop| {
                        let (compound, amount) = prop.split_once(':').unwrap();
                        (compound.trim().to_string(), amount.trim().parse().unwrap())
                    })
                    .collect()
            })
            .collect()
    }
}

impl Solution for Solution16 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let message = HashMap::from([
            ("children", 3),
            ("cats", 7),
            ("samoyeds", 2),
            ("pomeranians", 3),
            ("akitas", 0),
            ("vizslas", 0),
            ("goldfish", 5),
            ("trees", 3),
            ("cars", 2),
            ("perfumes", 1),
        ]);

        let sues = Self::parse(input);
        for (i, sue) in sues.into_iter().enumerate() {
            if sue.iter().all(|(prop, value)| message[prop.as_str()] == *value) {
                return (i + 1).to_result();
            }
        }
        panic!("No sue found")
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let message = HashMap::from([
            ("children", 3),
            ("cats", 7),
            ("samoyeds", 2),
            ("pomeranians", 3),
            ("akitas", 0),
            ("vizslas", 0),
            ("goldfish", 5),
            ("trees", 3),
            ("cars", 2),
            ("perfumes", 1),
        ]);

        let sues = Self::parse(input);
        for (i, sue) in sues.into_iter().enumerate() {
            if sue.iter().all(|(prop, value)| match prop.as_str() {
                "children" | "samoyeds" | "akitas" | "vizslas" | "cars" | "perfumes" => {
                    message[prop.as_str()] == *value
                }
                "cats" | "trees" => message[prop.as_str()] < *value,
                "pomeranians" | "goldfish" => message[prop.as_str()] > *value,
                _ => unreachable!(),
            }) {
                return (i + 1).to_result();
            }
        }
        panic!("No sue found")
    }
}
