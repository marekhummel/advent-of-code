use std::time::{Duration, Instant};

use crate::types::{ProblemInput, ProblemResult};

pub trait Solution {
    fn solve(&self, data: ProblemInput, version: u8) -> Option<(ProblemResult, Duration)> {
        let now = Instant::now();
        let result = match version {
            1 => self.solve_version01(data),
            2 => self.solve_version02(data),
            _ => panic!(),
        };
        result.map(|x| (x, now.elapsed()))
    }

    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult>;
    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult>;
}
