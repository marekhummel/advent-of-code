use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use rayon::iter::*;
pub struct Solution04;

impl Solution04 {
    const STEP_SIZE: usize = 1 << 15;

    fn find_hash(secret: &str, prefix: &str) -> usize {
        // Obviously works fine without chunks (~3s), but good test for parallelism
        for bn in (1usize..).step_by(Self::STEP_SIZE) {
            let digest = (bn..bn + Self::STEP_SIZE).into_par_iter().find_first(|n| {
                let digest = md5::compute(format!("{secret}{n}"));
                let digest_str = format!("{:x}", digest);
                digest_str.starts_with(prefix)
            });

            if let Some(n) = digest {
                return n;
            }
        }

        unreachable!()
    }
}

impl Solution for Solution04 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(1048970),
            ProblemResult::USize(254575),
            ProblemResult::USize(5714438),
            ProblemResult::USize(1038736),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let key = input.string();
        Self::find_hash(&key, "00000").to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let key = input.string();
        Self::find_hash(&key, "000000").to_result()
    }
}
