use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let polymer = parse(input);
    let reduced = reduce(polymer);
    reduced.len().into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let polymer = reduce(parse(input)); // Start with a reduced polymer cause these steps can always be taken.
    let units = polymer.iter().map(|(c, _)| c).unique().collect_vec();

    units
        .into_par_iter()
        .map(|unit| {
            let improved = polymer.iter().filter(|(c, _)| c != unit).cloned().collect();
            let reduced = reduce(improved);
            reduced.len()
        })
        .min()
        .unwrap()
        .into()
}

fn parse(input: ProblemInput) -> Vec<(u8, bool)> {
    input
        .string()
        .bytes()
        .map(|c| (c.to_ascii_lowercase() - b'a', c.is_ascii_uppercase()))
        .collect()
}

fn reduce(polymer: Vec<(u8, bool)>) -> Vec<(u8, bool)> {
    let mut reduced = Vec::new(); // Work as stack to avoid multiple iterations

    for (b, b_upper) in polymer {
        match reduced.last() {
            Some((a, a_upper)) if *a == b && *a_upper != b_upper => _ = reduced.pop(),
            _ => reduced.push((b, b_upper)),
        }
    }

    reduced
}
