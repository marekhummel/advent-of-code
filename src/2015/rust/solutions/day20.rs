use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution20;
impl Solution20 {
    fn divsum(n: u64, lazy: bool) -> u64 {
        if n == 0 || n == 1 {
            return n;
        }

        let root_n = (n as f32).sqrt() as u64;
        let sum: u64 = (1..=root_n)
            .filter(|d| n % d == 0)
            .flat_map(|d| if d * d == n { vec![d] } else { vec![d, n / d] })
            .filter(|div| !lazy || div * 50 >= n)
            .sum();

        sum
    }
}

impl Solution for Solution20 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U64(8),
            ProblemResult::U64(776160),
            ProblemResult::U64(8),
            ProblemResult::U64(786240),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let target = input.string().parse::<u64>().unwrap();
        (1..)
            .find(|house| Self::divsum(*house, false) * 10 >= target)
            .unwrap()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let target = input.string().parse::<u64>().unwrap();
        (1..)
            .find(|house| Self::divsum(*house, true) * 11 >= target)
            .unwrap()
            .to_result()
    }
}
