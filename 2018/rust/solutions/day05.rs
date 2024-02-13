use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct Solution05;
impl Solution05 {
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
}

impl Solution for Solution05 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let polymer = Self::parse(input);
        let reduced = Self::reduce(polymer);
        reduced.len().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let polymer = Self::reduce(Self::parse(input)); // Start with a reduced polymer cause these steps can always be taken.
        let units = polymer.iter().map(|(c, _)| c).unique().collect_vec();

        units
            .into_par_iter()
            .map(|unit| {
                let improved = polymer.iter().filter(|(c, _)| c != unit).cloned().collect();
                let reduced = Self::reduce(improved);
                reduced.len()
            })
            .min()
            .unwrap()
            .to_result()
    }
}
