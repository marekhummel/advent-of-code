use std::time::{Duration, Instant};

use crate::types::{ProblemInput, ProblemResult};

pub trait Solution {
    fn solve(&self, data: ProblemInput, version: u8, is_sample: bool) -> (ProblemResult, Duration) {
        let now = Instant::now();
        let result = match version {
            1 => self.solve_version01(data, is_sample),
            2 => self.solve_version02(data, is_sample),
            _ => panic!("Invalid Version Number '{version}'"),
        };
        (result, now.elapsed())
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult;
    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult;
}
