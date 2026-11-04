use std::collections::HashMap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution14;
impl Solution14 {}

impl Solution for Solution14 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U64(165),
            ProblemResult::U64(8566770985168),
            ProblemResult::U64(208),
            ProblemResult::U64(4832039794082),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut memory = HashMap::new();
        let mut mask = Vec::new();

        for line in input.lines() {
            if let Some(mask_str) = line.strip_prefix("mask = ") {
                // Update mask
                mask.clear();
                for (pos, bit) in mask_str.bytes().rev().enumerate() {
                    match bit {
                        b'0' => mask.push((pos, false)),
                        b'1' => mask.push((pos, true)),
                        b'X' => (),
                        _ => unreachable!(),
                    }
                }
            } else {
                // Parse assignment
                let (lhs, rhs) = line.split_once("] = ").unwrap();
                let address: u64 = lhs.trim_start_matches("mem[").parse().unwrap();
                let mut value: u64 = rhs.parse().unwrap();

                // Update value with mask
                for (pos, set) in &mask {
                    value = match set {
                        true => value | (1 << pos),   // Set 1
                        false => value & !(1 << pos), // Set 0
                    }
                }

                memory.insert(address, value);
            }
        }

        memory.values().sum::<u64>().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut memory = HashMap::new();
        let mut mask = Vec::new();

        for line in input.lines() {
            if let Some(mask_str) = line.strip_prefix("mask = ") {
                // Update mask
                mask.clear();
                for (pos, bit) in mask_str.bytes().rev().enumerate() {
                    match bit {
                        b'0' => (),
                        b'1' => mask.push((pos, false)),
                        b'X' => mask.push((pos, true)),
                        _ => unreachable!(),
                    }
                }
            } else {
                // Get address and value
                let (lhs, rhs) = line.split_once("] = ").unwrap();
                let value: u64 = rhs.parse().unwrap();
                let mut address: u64 = lhs.trim_start_matches("mem[").parse().unwrap();

                // Update all ones from mask in address
                for (pos, set) in &mask {
                    if !set {
                        address |= 1 << pos;
                    }
                }

                // Check Xs in mask
                let xs = mask.iter().filter_map(|(pos, is_x)| is_x.then_some(pos)).collect_vec();

                if xs.is_empty() {
                    // Without Xs, just insert value
                    memory.insert(address, value);
                } else {
                    // Given n Xs, loop over all permutations for X (basically all integers from 0 to n - 1)
                    for floating in vec![[false, true]; xs.len()].into_iter().multi_cartesian_product() {
                        let mut float_address = address;
                        for (pos, bit) in xs.iter().zip_eq(floating) {
                            float_address = match bit {
                                true => float_address | (1 << **pos),   // Set 1
                                false => float_address & !(1 << **pos), // Set 0
                            }
                        }

                        memory.insert(float_address, value);
                    }
                }
            }
        }

        // Approach works, because memory.len() ~ 80k, which is still very low. Otherwise, we would have to devise a key
        // that identifies ranges of memory, and continously split that space when needed.
        memory.values().sum::<u64>().to_result()
    }
}
