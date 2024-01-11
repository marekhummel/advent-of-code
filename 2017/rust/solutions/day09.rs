use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution09;
impl Solution09 {}

impl Solution for Solution09 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
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

        score.into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
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

        garbage_count.into_some()
    }
}
