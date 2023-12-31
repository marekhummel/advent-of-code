use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;
pub struct Solution02;

impl Solution02 {
    fn parse(input: ProblemInput) -> Vec<(u32, u32, u32)> {
        input
            .lines()
            .into_iter()
            .map(|line| {
                line.split('x')
                    .map(|d| d.parse::<u32>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_vec()
    }
}

impl Solution for Solution02 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        Self::parse(input)
            .into_iter()
            .map(|(l, w, h)| (l * w, l * h, w * h))
            .map(|(a, b, c)| 2 * a + 2 * b + 2 * c + a.min(b.min(c)))
            .sum::<u32>()
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        Self::parse(input)
            .into_iter()
            .map(|(l, w, h)| (l + w, l + h, w + h, l * w * h))
            .map(|(a, b, c, v)| 2 * a.min(b.min(c)) + v)
            .sum::<u32>()
            .into_some()
    }
}
