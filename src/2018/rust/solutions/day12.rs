use std::collections::HashMap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution12;
impl Solution12 {
    fn parse(input: ProblemInput) -> (Vec<bool>, HashMap<Vec<bool>, bool>) {
        let lines = input.lines();

        let initial_state = lines[0]
            .trim_start_matches("initial state:")
            .trim()
            .chars()
            .map(|c| c == '#')
            .collect_vec();

        let rules = lines[2..]
            .iter()
            .map(|l| {
                let (pattern_str, result_str) = l.split_once("=>").unwrap();
                let pattern = pattern_str.trim().chars().map(|c| c == '#').collect_vec();
                let result = result_str.trim() == "#";
                (pattern, result)
            })
            .collect();

        (initial_state, rules)
    }

    fn next_gen(gen: &[i64], rules: &HashMap<Vec<bool>, bool>) -> Vec<i64> {
        let mut new_gen = Vec::new();
        let min = gen[0] - 3;
        let max = gen[gen.len() - 1] + 3;
        let window = [-2, -1, 0, 1, 2];

        for i in min..=max {
            let pattern = window.map(|o| gen.binary_search(&(i + o)).is_ok()).to_vec();
            if *rules.get(&pattern).unwrap_or(&false) {
                new_gen.push(i);
            }
        }

        new_gen
    }
}

impl Solution for Solution12 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I64(325),
            ProblemResult::I64(1816),
            ProblemResult::I64(999999999374),
            ProblemResult::I64(399999999957),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (initial_state, rules) = Self::parse(input);

        let mut state = initial_state
            .into_iter()
            .enumerate()
            .filter_map(|(i, b)| b.then_some(i as i64))
            .collect_vec();

        for _ in 0..20 {
            state = Self::next_gen(&state, &rules);
        }

        state.into_iter().sum::<i64>().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (initial_state, rules) = Self::parse(input);

        let mut state = initial_state
            .into_iter()
            .enumerate()
            .filter_map(|(i, b)| b.then_some(i as i64))
            .collect_vec();

        let mut seen = HashMap::from([(state.clone(), (0usize, state.iter().sum::<i64>()))]);
        for g in 1.. {
            state = Self::next_gen(&state, &rules);

            // Correct alignment to catch repetitions
            let state_key = state.iter().map(|i| i - state[0]).collect_vec();
            let score = state.iter().sum::<i64>();

            // Check if state has occured before
            if let Some((last_g, last_score)) = seen.insert(state_key, (g, score)) {
                // Find offset in loop to take for 50 billionth step, and compute score until then
                let period = g - last_g;
                let offset = (50_000_000_000 - last_g) % period;
                let final_state = (0..offset).fold(state, |gen, _| Self::next_gen(&gen, &rules));

                // Between each period, the score shifts, count increments to the 50 billionth step
                let score_inc = ((50_000_000_000 - last_g) / period - 1) as i64 * (score - last_score);
                return (final_state.into_iter().sum::<i64>() + score_inc).to_result();
            }
        }

        unreachable!()
    }
}
