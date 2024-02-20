use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution05;
impl Solution05 {
    fn jump<F: Fn(i32) -> i32>(jumps: &mut [i32], update_jump: F) -> u32 {
        let mut pc = 0;
        let mut steps = 0;
        while pc < jumps.len() {
            let new_pc = (pc as i32 + jumps[pc]) as usize;
            *jumps.get_mut(pc).unwrap() = update_jump(jumps[pc]);
            pc = new_pc;

            steps += 1;
        }

        steps
    }
}

impl Solution for Solution05 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut jumps = input.lines().into_iter().parsed::<i32>().collect_vec();

        let steps = Self::jump(&mut jumps, |offset| offset + 1);
        steps.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut jumps = input.lines().into_iter().parsed::<i32>().collect_vec();

        let steps = Self::jump(&mut jumps, |offset| offset + if offset >= 3 { -1 } else { 1 });
        steps.to_result()
    }
}
