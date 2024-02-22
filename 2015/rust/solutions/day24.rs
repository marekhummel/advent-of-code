use std::cmp::Ordering;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

type Present = u128;

pub struct Solution24;
impl Solution24 {
    fn parse(input: ProblemInput) -> Vec<Present> {
        input.lines().into_iter().parsed().sorted().rev().collect()
    }

    fn pack(presents: &[Present], weight: Present, front_compartment: Vec<Present>) -> Option<Vec<Present>> {
        match front_compartment.iter().sum::<Present>().cmp(&weight) {
            Ordering::Equal => return Some(front_compartment),
            Ordering::Greater => return None,
            Ordering::Less => (),
        }

        if presents.is_empty() {
            return None;
        }

        let mut possible_packings: Vec<Vec<Present>> = Vec::new();
        possible_packings.extend(Self::pack(&presents[1..], weight, front_compartment.clone()));

        let mut new_front = front_compartment;
        new_front.push(presents[0]);
        possible_packings.extend(Self::pack(&presents[1..], weight, new_front));

        possible_packings
            .into_iter()
            .min_by_key(|front| (front.len(), front.iter().product::<Present>()))
    }
}

impl Solution for Solution24 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let presents = Self::parse(input);
        let weight_per_group = presents.iter().sum::<Present>() / 3;

        // Takes about 14secs in debug
        let best_front = Self::pack(&presents, weight_per_group, vec![]).unwrap();
        best_front.iter().product::<Present>().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let presents = Self::parse(input);
        let weight_per_group = presents.iter().sum::<Present>() / 4;

        let best_front = Self::pack(&presents, weight_per_group, vec![]).unwrap();
        best_front.iter().product::<Present>().to_result()
    }
}
