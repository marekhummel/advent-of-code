use crate::prelude::types::{ProblemInput, ProblemResult};

pub type Part = fn(ProblemInput, bool) -> ProblemResult;

#[derive(Clone, Copy)]
pub struct Solution {
    pub part1: Part,
    pub part2: Part,
}

/// Register a day module, checking both function signatures against `Part`.
#[macro_export]
macro_rules! solution {
    ($day:ident) => {
        $crate::prelude::solution::Solution {
            part1: $day::part1,
            part2: $day::part2,
        }
    };
}
