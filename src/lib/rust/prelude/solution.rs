use crate::prelude::types::{ProblemInput, ProblemResult};

pub trait Solution {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult;
    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult;
}
