use std::collections::VecDeque;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution09;
impl Solution09 {
    fn find_invalid(nums: &[u32], preamble_len: usize) -> u32 {
        let mut window = VecDeque::new();
        for num in nums {
            if window.len() == preamble_len {
                if !window.iter().tuple_combinations().any(|(&a, &b)| a + b == *num) {
                    return *num;
                }

                window.pop_front();
            }

            window.push_back(num);
        }

        unreachable!()
    }
}

impl Solution for Solution09 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let preamble_len = if is_sample { 5 } else { 25 };

        let nums = input.lines().into_iter().parsed().collect_vec();
        Self::find_invalid(&nums, preamble_len).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let preamble_len = if is_sample { 5 } else { 25 };

        let nums = input.lines().into_iter().parsed().collect_vec();
        let invalid = Self::find_invalid(&nums, preamble_len);

        for i in 0..nums.len() - 1 {
            let mut j = i;
            let mut sum = nums[i];
            while sum < invalid {
                j += 1;
                sum += nums[j];
            }

            if sum == invalid {
                let (small, large) = nums[i..=j].iter().minmax().into_option().unwrap();
                return (small + large).to_result();
            }
        }

        unreachable!()
    }
}
