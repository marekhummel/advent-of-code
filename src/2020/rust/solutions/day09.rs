use std::collections::VecDeque;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, is_sample: bool) -> ProblemResult {
    let preamble_len = if is_sample { 5 } else { 25 };

    let nums = input.lines().into_iter().parsed().collect_vec();
    find_invalid(&nums, preamble_len).into()
}

pub fn part2(input: ProblemInput, is_sample: bool) -> ProblemResult {
    let preamble_len = if is_sample { 5 } else { 25 };

    let nums = input.lines().into_iter().parsed().collect_vec();
    let invalid = find_invalid(&nums, preamble_len);

    for i in 0..nums.len() - 1 {
        let mut j = i;
        let mut sum = nums[i];
        while sum < invalid {
            j += 1;
            sum += nums[j];
        }

        if sum == invalid {
            let (small, large) = nums[i..=j].iter().minmax().into_option().unwrap();
            return (small + large).into();
        }
    }

    unreachable!()
}

fn find_invalid(nums: &[u32], preamble_len: usize) -> u32 {
    let mut window = VecDeque::new();
    for num in nums {
        if window.len() == preamble_len {
            if !window.iter().array_combinations().any(|[&a, &b]| a + b == *num) {
                return *num;
            }

            window.pop_front();
        }

        window.push_back(num);
    }

    unreachable!()
}
