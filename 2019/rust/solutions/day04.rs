use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution04;
impl Solution04 {}

impl Solution for Solution04 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::NoSample,
            ProblemResult::I32(1605),
            ProblemResult::NoSample,
            ProblemResult::I32(1102),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let (low, high) = input.string().split('-').parsed::<u32>().collect_tuple().unwrap();

        let mut total = 0;
        for pwd_num in low..=high {
            let pwd = pwd_num.to_string();
            let non_decreasing = pwd.bytes().tuple_windows().all(|(d1, d2)| d1 <= d2);
            let double = pwd.bytes().group_by(|d| *d).into_iter().any(|(_, g)| g.count() > 1);

            if non_decreasing && double {
                total += 1;
            }
        }

        total.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let (low, high) = input.string().split('-').parsed::<u32>().collect_tuple().unwrap();

        let mut total = 0;
        for pwd_num in low..=high {
            let pwd = pwd_num.to_string();
            let non_decreasing = pwd.bytes().tuple_windows().all(|(d1, d2)| d1 <= d2);
            let double = pwd.bytes().group_by(|d| *d).into_iter().any(|(_, g)| g.count() == 2);

            if non_decreasing && double {
                total += 1;
            }
        }

        total.to_result()
    }
}
