use aoc_lib::math;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution13;
impl Solution13 {}

impl Solution for Solution13 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let notes = input.lines();
        let depart = notes[0].parse::<u64>().unwrap();
        let busses = notes[1].split(',').filter_map(|b| b.parse::<u64>().ok()).collect_vec();

        let next_departures = busses.into_iter().map(|b| (b, b - depart % b)).collect_vec();

        next_departures
            .into_iter()
            .min_by_key(|(_, wait)| *wait)
            .map(|(b, wait)| b * wait)
            .unwrap()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let constraints = input.lines()[1]
            .split(',')
            .enumerate()
            .filter_map(|(off, b)| b.parse::<i64>().ok().map(|id| (id, off as i64)))
            .collect_vec();

        // (t + off) % id == 0
        // t + off = 0    mod id
        // t = id - off   mod id
        let residues = constraints.iter().map(|(id, off)| id - off).collect_vec();
        let modulii = constraints.iter().map(|(id, _)| *id).collect_vec();
        math::chinese_remainder(&residues, &modulii).unwrap().to_result()
    }
}
