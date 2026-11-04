use std::collections::VecDeque;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution24;
impl Solution24 {
    const PARAMS: [(i32, i32, i32); 14] = [
        (11, 1, 7),
        (14, 1, 8),
        (10, 1, 16),
        (14, 1, 8),
        (-8, 26, 3),
        (14, 1, 12),
        (-11, 26, 1),
        (10, 1, 8),
        (-6, 26, 8),
        (-9, 26, 14),
        (12, 1, 4),
        (-5, 26, 14),
        (-4, 26, 15),
        (-9, 26, 6), // z == 18
    ];

    fn valid_model_numbers() -> Vec<u64> {
        // Each evaluation of a digit is essentially the following, where z is accumulated over all digits:
        //     x = (z % 26 + a != w) as i64; // 0 or 1
        //     z = (z / b) * (25 * x + 1) + ((w + c) * x);
        //
        // Note a few things:
        //    - w in 1..=9, b in [1, 26], c > 0, z > 0.
        //    - We can divide by 26 7 times, thus we can afford to have x == 1 for 7 times
        //      and still achieve z == 0 in the end.
        //    - Since a > 10 when b == 1, x will always be 1.
        // => So we have to fix w when b == 26 to get x == 0, and freely choose when b == 1.
        let mut queue = VecDeque::from([(0u64, 0usize, 0i32)]);

        let mut valids = Vec::new();
        while let Some((model_no, length, z)) = queue.pop_front() {
            if length == 14 {
                if z == 0 {
                    valids.push(model_no);
                }
                continue;
            }

            let (a, b, c) = Self::PARAMS[length];
            if b == 1 {
                // When b == 1 (no division) we can test out all w
                for w in 1..=9i32 {
                    // x is always 1 given the values of a
                    assert_ne!(z % 26 + a, w);

                    let next_model_no = model_no * 10 + w as u64;
                    queue.push_back((next_model_no, length + 1, (z / b) * 26 + (w + c)));
                }
            } else {
                // When b == 26 we will divide, thus we need x to be 0.
                let w = (z % 26 + a) as u64;
                if (1..=9).contains(&w) {
                    let next_model_no = model_no * 10 + w;
                    queue.push_back((next_model_no, length + 1, z / b));
                }
            }
        }

        valids
    }
}

impl Solution for Solution24 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::NoSample,
            ProblemResult::U64(95299897999897),
            ProblemResult::NoSample,
            ProblemResult::U64(31111121382151),
        ]
    }

    #[allow(clippy::assign_op_pattern)]
    fn solve_version01(&self, _input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let valids = Self::valid_model_numbers();
        let highest = valids.into_iter().max().unwrap();
        highest.to_result()
    }

    fn solve_version02(&self, _input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let valids = Self::valid_model_numbers();
        let lowest = valids.into_iter().min().unwrap();
        lowest.to_result()
    }
}
