use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution03;
impl Solution03 {
    fn rating(mut nums: Vec<Vec<bool>>, keep_ones: bool) -> u32 {
        let mut i = 0;
        while nums.len() > 1 {
            let ones = nums.iter().filter(|num| num[i]).count();
            let keep = if ones * 2 >= nums.len() { keep_ones } else { !keep_ones };
            nums.retain(|num| num[i] == keep);
            i += 1;
        }
        nums[0].iter().fold(0, |acc, b| (acc << 1) + (*b as u32))
    }
}

impl Solution for Solution03 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(198),
            ProblemResult::U32(3885894),
            ProblemResult::U32(230),
            ProblemResult::U32(4375225),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let digits = input.grid().transpose();
        let majorities = digits
            .rows
            .into_iter()
            .map(|ds| ds.iter().filter(|d| **d == '1').count() >= digits.size.width / 2)
            .collect_vec();
        let gamma_rate = majorities.iter().fold(0, |acc, b| (acc << 1) + (*b as u32));
        let epsilon_rate = majorities.iter().fold(0, |acc, b| (acc << 1) + (!*b as u32));

        (gamma_rate * epsilon_rate).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let diagnostics = input.grid().map_elements(|c| *c == '1').rows;

        let oxygen_generator_rating = Self::rating(diagnostics.clone(), true);
        let co2_scrubber_rating = Self::rating(diagnostics.clone(), false);
        (oxygen_generator_rating * co2_scrubber_rating).to_result()
    }
}
