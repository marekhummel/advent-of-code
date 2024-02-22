use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
pub struct Solution19;
impl Solution19 {
    fn josephus(n: u32) -> u32 {
        match (n, n & 1) {
            (1, _) => 1,
            (_, 0) => 2 * Self::josephus(n / 2) - 1,
            (_, 1) => 2 * Self::josephus(n / 2) + 1,
            _ => unreachable!(),
        }
    }

    fn josephus2(n: usize) -> usize {
        let mut power3 = 1;
        while power3 * 3 < n {
            power3 *= 3;
        }

        if n <= 2 * power3 {
            n - power3
        } else {
            power3 + 2 * (n - 2 * power3)
        }
    }

    // Use this to find a pattern for n up to 100. Pattern emerges which leads to implementation of
    // josephus2(). Basically at every power of three the pattern repeats. Let n be the power of three,
    // then the solutions for n+i with i = 1..=n is just i, and after that the solutions increment by 2.
    #[allow(dead_code)]
    fn josephus_sim(n: usize) -> u32 {
        let mut elves = Vec::from_iter(1..=n as u32);
        while elves.len() > 1 {
            // println!("{:?} - {} steals from {}", elves, turn, oppo);
            let oppo = elves.len() / 2;
            elves.remove(oppo);
            elves = [&elves[1..], &elves[0..1]].concat();
        }

        elves[0]
    }
}

impl Solution for Solution19 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(3),
            ProblemResult::U32(1808357),
            ProblemResult::USize(2),
            ProblemResult::USize(1407007),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        Self::josephus(input.string().parse().unwrap()).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        Self::josephus2(input.string().parse().unwrap()).to_result()
    }
}
