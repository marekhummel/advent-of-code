use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution20;
impl Solution20 {
    fn parse(input: ProblemInput) -> Vec<isize> {
        input.lines().into_iter().parsed().collect()
    }

    fn initial_arrangement(numbers: &[isize]) -> Vec<[usize; 2]> {
        let len = numbers.len();
        (0..len).map(|i| [(i + len - 1) % len, (i + 1) % len]).collect()
    }

    fn mix(numbers: &[isize], arrangement: &mut [[usize; 2]]) {
        for (idx, num) in numbers.iter().enumerate() {
            // Compute new index
            let steps = (*num).rem_euclid(numbers.len() as isize - 1);
            let new_idx = (0..steps).fold(idx, |curr, _| arrangement[curr][1]);

            if idx == new_idx {
                continue;
            }

            // Remove at old
            let [prev, next] = arrangement[idx];
            arrangement[prev][1] = next;
            arrangement[next][0] = prev;

            // Insert at new
            let new_next = arrangement[new_idx][1];
            arrangement[idx] = [new_idx, new_next];
            arrangement[new_idx][1] = idx;
            arrangement[new_next][0] = idx;
        }
    }

    fn get_coordinates(numbers: &[isize], arrangement: &[[usize; 2]]) -> isize {
        let zero_idx = numbers.iter().position(|n| *n == 0).unwrap();
        let chain = (0..).scan(zero_idx, |curr, _| {
            let number = numbers[*curr];
            *curr = arrangement[*curr][1];
            Some(number)
        });

        // Iterator with step returns 0th, 1000th, 2000th etc. number, hence the skip(1)
        chain.step_by(1000).skip(1).take(3).sum::<isize>()
    }

    #[allow(dead_code)]
    fn print(numbers: &[isize], arrangement: &[[usize; 2]]) {
        println!(
            "{:?}\n",
            (0..numbers.len())
                .scan(0, |curr, _| {
                    let number = numbers[*curr];
                    *curr = arrangement[*curr][1];
                    Some(number)
                })
                .collect_vec()
        );
    }
}

impl Solution for Solution20 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::ISize(3),
            ProblemResult::ISize(8372),
            ProblemResult::ISize(1623178306),
            ProblemResult::ISize(7865110481723),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let numbers = Self::parse(input);
        let mut arrangement = Self::initial_arrangement(&numbers);

        Self::mix(&numbers, &mut arrangement);
        Self::get_coordinates(&numbers, &arrangement).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let numbers = Self::parse(input).into_iter().map(|n| n * 811589153).collect_vec();
        let mut arrangement = Self::initial_arrangement(&numbers);

        (0..10).for_each(|_| Self::mix(&numbers, &mut arrangement));
        Self::get_coordinates(&numbers, &arrangement).to_result()
    }
}
