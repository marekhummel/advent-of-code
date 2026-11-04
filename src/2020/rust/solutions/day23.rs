use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use num::{Integer, Unsigned};

pub struct Solution23;
impl Solution23 {
    fn crab_move(cups: &mut Vec<u8>) {
        let max_cup = cups.len() as u8;

        // Pick
        let picks = cups.drain(1..4).collect_vec();

        // Determine destincation
        let destination_cup = Self::destination(cups[0], max_cup, &picks);

        // Find and insert
        let destination_pos = cups.iter().position(|c| *c == destination_cup).unwrap() + 1;
        cups.splice(destination_pos..destination_pos, picks);

        // Next cup
        cups.rotate_left(1);
    }

    fn destination<T: Copy + Integer + Unsigned>(current: T, cups: T, picks: &[T]) -> T {
        let two = T::one() + T::one();
        let mut destination = current;
        loop {
            destination = (destination - two + cups) % cups + T::one();
            if !picks.contains(&destination) {
                break;
            }
        }
        destination
    }
}

impl Solution for Solution23 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::String("67384529".to_string()),
            ProblemResult::String("52864379".to_string()),
            ProblemResult::USize(149245887792),
            ProblemResult::USize(11591415792),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut circle = input.string().chars().parsed().collect();
        (0..100).for_each(|_| Self::crab_move(&mut circle));

        let cup1 = circle.iter().position(|c| *c == 1).unwrap();
        circle.rotate_left(cup1 + 1);
        circle.pop();

        circle.into_iter().map(|c| c.to_string()).join("").to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let num_cups = 1_000_000;
        let iterations = 10_000_000;

        // Create linked list
        let mut labeling = input.string().chars().parsed::<usize>().collect_vec();
        labeling.extend(labeling.len() + 1..=num_cups);
        let mut next = vec![0; num_cups + 1].into_boxed_slice();
        for (c, n) in labeling.iter().circular_tuple_windows() {
            next[*c] = *n;
        }

        // Iterate
        let mut cup = labeling[0];
        for _ in 0..iterations {
            // Pick cups
            let pick1 = next[cup];
            let pick2 = next[pick1];
            let pick3 = next[pick2];

            // Find destination
            let destination = Self::destination(cup, num_cups, &[pick1, pick2, pick3]);

            // Insert
            next[cup] = next[pick3]; // Close the link between picked cups
            next[pick3] = next[destination]; // Whatever was after dest is now after our picks
            next[destination] = pick1; // After our dest are now the picks

            // Advance
            cup = next[cup];
        }

        (next[1] * next[next[1]]).to_result()
    }
}
