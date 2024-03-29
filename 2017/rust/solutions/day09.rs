use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution09;
impl Solution09 {}

impl Solution for Solution09 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I32(9),
            ProblemResult::I32(14190),
            ProblemResult::I32(8),
            ProblemResult::I32(7053),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let stream = input.string();

        let mut garbage = false;
        let mut negate = false;
        let mut depth = 0;
        let mut score = 0;

        for c in stream.chars() {
            if garbage {
                if negate {
                    negate = false;
                    continue;
                } else if c == '>' {
                    garbage = false;
                } else if c == '!' {
                    negate = true;
                }
            } else {
                match c {
                    '<' => garbage = true,
                    '{' => {
                        depth += 1;
                        score += depth;
                    }
                    '}' => {
                        depth -= 1;
                    }
                    _ => (),
                }
            }
        }

        score.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let stream = input.string();

        let mut garbage = false;
        let mut negate = false;
        let mut garbage_count = 0;

        for c in stream.chars() {
            if garbage {
                if negate {
                    negate = false;
                    continue;
                } else if c == '>' {
                    garbage = false;
                } else if c == '!' {
                    negate = true;
                } else {
                    garbage_count += 1;
                }
            } else if c == '<' {
                garbage = true;
            }
        }

        garbage_count.to_result()
    }
}
