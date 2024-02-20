use aoc_lib::iterator::ParsedExt;
use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution15;
impl Solution15 {
    fn play(starting_nums: &[u32], limit: u32) -> u32 {
        // Use vec instead of hashmap, cause its still feasible for the given limit and 2x faster
        let mut history = vec![u32::MAX; limit as usize];
        let (last, spoken) = starting_nums.split_last().unwrap();
        for (turn, &num) in spoken.iter().enumerate() {
            history[num as usize] = turn as u32;
        }

        let mut last_num = *last as usize;
        for turn in starting_nums.len() as u32..limit {
            // Use saturating sub instead of Option for speed, will give 0 if first spoken due to u32::MAX
            let speak = (turn - 1).saturating_sub(history[last_num]);
            history[last_num] = turn - 1;
            last_num = speak as usize;
        }

        last_num as u32
    }
}

impl Solution for Solution15 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let starting_nums = input.string().split(',').parsed().collect_vec();
        Self::play(&starting_nums, 2020).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let starting_nums = input.string().split(',').parsed().collect_vec();
        Self::play(&starting_nums, 30_000_000).to_result()
    }
}
