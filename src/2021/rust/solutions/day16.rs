use std::cmp::Ordering;

use aoc_lib::math;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution16;
impl Solution16 {
    fn parse_hex_string(hex_string: &str) -> Vec<bool> {
        hex_string
            .chars()
            .flat_map(|c| math::bits::<4>(u128::from_str_radix(&c.to_string(), 16).unwrap()))
            .collect()
    }

    fn eval(bits: &mut Vec<bool>) -> (u64, u64) {
        let version = Self::to_num(bits.drain(..3).collect());
        let type_id = Self::to_num(bits.drain(..3).collect());

        if type_id == 4 {
            // Literal
            let mut literal_value: Vec<bool> = Vec::new();
            loop {
                let chunk = bits.drain(..5).collect_vec();
                literal_value.extend(&chunk[1..]);
                if !chunk[0] {
                    break;
                }
            }
            let num = Self::to_num(literal_value);
            (version, num)
        } else {
            // Operator
            let length_type_id = bits.drain(..1).next().unwrap();
            let mut subpackets = Vec::new();
            if length_type_id {
                // Number of sub-packets
                let num_subpackets = Self::to_num(bits.drain(..11).collect());
                subpackets.extend((0..num_subpackets).map(|_| Self::eval(bits)));
            } else {
                // Total length in bits
                let subpacket_length = Self::to_num(bits.drain(..15).collect());
                let mut bits_read = 0;
                while bits_read < subpacket_length {
                    let len_before = bits.len();
                    subpackets.push(Self::eval(bits));
                    bits_read += (len_before - bits.len()) as u64;
                }
            }

            let (versions, nums): (Vec<_>, Vec<_>) = subpackets.into_iter().unzip();
            let value = match type_id {
                0 => nums.into_iter().sum(),
                1 => nums.into_iter().product(),
                2 => nums.into_iter().min().unwrap(),
                3 => nums.into_iter().max().unwrap(),
                5..=7 => {
                    let (n1, n2) = nums.into_iter().collect_tuple().unwrap();
                    match (type_id, n1.cmp(&n2)) {
                        (5, Ordering::Greater) => 1,
                        (6, Ordering::Less) => 1,
                        (7, Ordering::Equal) => 1,
                        _ => 0,
                    }
                }

                _ => unreachable!(),
            };

            (version + versions.iter().sum::<u64>(), value)
        }
    }

    fn to_num(bits: Vec<bool>) -> u64 {
        bits.iter().fold(0, |num, bit| (num << 1) + *bit as u64)
    }
}

impl Solution for Solution16 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U64(31),
            ProblemResult::U64(996),
            ProblemResult::U64(1),
            ProblemResult::U64(96257984154),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut bits = Self::parse_hex_string(&input.string());
        let (version_sum, _) = Self::eval(&mut bits);
        version_sum.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut bits = Self::parse_hex_string(&input.string());
        let (_, result) = Self::eval(&mut bits);
        result.to_result()
    }
}
