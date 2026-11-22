use std::collections::HashMap;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut adapters = input.lines().into_iter().parsed::<u16>().sorted().collect_vec();
    adapters.insert(0, 0);
    adapters.push(adapters[adapters.len() - 1] + 3);

    let deltas = adapters.into_iter().tuple_windows().map(|(a, b)| b - a).collect_vec();
    let counts = deltas.into_iter().counts();

    (counts[&1] * counts[&3]).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let adapters = input.lines().into_iter().parsed::<u16>().sorted().collect_vec();
    let output = adapters[adapters.len() - 1] + 3;
    combinations(&adapters, output, 0, &mut HashMap::new()).into()
}

fn combinations(adapters: &[u16], output: u16, joltage: u16, cache: &mut HashMap<u16, u64>) -> u64 {
    // Memoization
    if let Some(combs) = cache.get(&joltage) {
        return *combs;
    }

    // Check three offsets, recurse if adapter exists
    let mut total_combs = 0;
    for delta in [1, 2, 3] {
        if joltage + delta == output {
            total_combs += 1;
        } else if adapters.binary_search(&(joltage + delta)).is_ok() {
            total_combs += combinations(adapters, output, joltage + delta, cache);
        }
    }

    cache.insert(joltage, total_combs);
    total_combs
}
