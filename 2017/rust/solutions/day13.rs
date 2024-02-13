use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution13;
impl Solution13 {
    fn parse(input: ProblemInput) -> Vec<(usize, usize)> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (d, r) = l.split_once(": ").unwrap();
                (d.parse().unwrap(), r.parse().unwrap())
            })
            .collect()
    }
}

impl Solution for Solution13 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let layers = Self::parse(input);

        layers
            .into_iter()
            .filter(|(depth, range)| depth % ((range - 1) * 2) == 0)
            .map(|(depth, range)| depth * range)
            .sum::<usize>()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let layers = Self::parse(input)
            .into_iter()
            .map(|(depth, range)| (depth, (range - 1) * 2))
            .collect_vec();

        (0..)
            .find(|delay| layers.iter().all(|(depth, period)| (*depth + delay) % period != 0))
            .unwrap()
            .to_result()
    }
}
