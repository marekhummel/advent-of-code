use crate::prelude::types::{ProblemInput, ProblemResult};

pub trait Solution {
    fn solve_version01(&self, _input: ProblemInput, _is_sample: bool) -> ProblemResult {
        ProblemResult::NotImplemented
    }
    fn solve_version02(&self, _input: ProblemInput, _is_sample: bool) -> ProblemResult {
        ProblemResult::NotImplemented
    }
}
