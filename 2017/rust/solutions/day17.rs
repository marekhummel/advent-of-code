use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
pub struct Solution17;
impl Solution17 {}

impl Solution for Solution17 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let steps: usize = input.string().parse().unwrap();
        let mut buffer = vec![0];
        let mut idx = 0;

        for n in 1..=2017 {
            idx = (idx + steps) % n + 1;
            buffer.insert(idx, n);
        }

        buffer[(idx + 1) % buffer.len()].to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let steps: usize = input.string().parse().unwrap();
        let mut idx = 0;
        let mut result = 0;
        for n in 1..=50_000_000 {
            idx = (idx + steps) % n + 1;
            if idx == 1 {
                // 0 will always stay at position 0, so just check if value afterwards changes.
                result = n;
            }
        }

        result.to_result()
    }
}
