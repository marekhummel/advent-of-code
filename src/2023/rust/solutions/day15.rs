use itertools::Itertools;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution15;
impl Solution15 {
    fn parse(input: ProblemInput) -> Vec<String> {
        input.string().split(',').map(String::from).collect_vec()
    }

    fn split_step(step: &str) -> (String, String, Option<u8>) {
        let mut step_iter = step.chars();
        let label = step_iter.peeking_take_while(|c| *c != '-' && *c != '=').join("");
        let action = step_iter.next().unwrap();
        match action {
            '-' => (label, action.to_string(), None),
            '=' => (label, action.to_string(), Some(step_iter.join("").parse().unwrap())),
            _ => unreachable!(),
        }
    }

    fn hash(value: &str) -> u8 {
        value.chars().fold(0u16, |digest, c| ((digest + c as u16) * 17) % 256) as u8
    }
}

impl Solution for Solution15 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        Self::parse(input)
            .iter()
            .map(|s| Self::hash(s) as u32)
            .sum::<u32>()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut boxes: [Vec<(String, u8)>; 256] = vec![Vec::new(); 256].try_into().unwrap();

        for step in Self::parse(input) {
            let (label, action, focal) = Self::split_step(&step);
            let b = &mut boxes[Self::hash(&label) as usize];
            let slot_no = b.iter().position(|(lens, _)| *lens == label);

            match (slot_no, action.as_str()) {
                (Some(pos), "=") => *b.get_mut(pos).unwrap() = (label, focal.unwrap()),
                (Some(pos), "-") => _ = b.remove(pos),
                (None, "=") => b.push((label, focal.unwrap())),
                (None, "-") => (),
                _ => unreachable!(),
            }
        }

        let focus_power = boxes
            .iter()
            .enumerate()
            .flat_map(|(box_no, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(|(slot_no, (_, focal))| (box_no as u32 + 1) * (slot_no as u32 + 1) * (*focal as u32))
                    .collect_vec()
            })
            .sum::<u32>();
        focus_power.to_result()
    }
}
