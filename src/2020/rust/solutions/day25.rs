use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution25;
impl Solution25 {}

struct TransformIterator {
    value: u64,
    subject_number: u64,
}

impl TransformIterator {
    fn new(subject_number: u64) -> Self {
        Self {
            value: 1,
            subject_number,
        }
    }
}

impl Iterator for TransformIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * self.subject_number) % 20201227;
        Some(self.value)
    }
}

impl Solution for Solution25 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U64(14897079),
            ProblemResult::U64(7936032),
            ProblemResult::NoPartTwo,
            ProblemResult::NoPartTwo,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (pub_door, pub_card) = input.lines().into_iter().parsed::<u64>().collect_tuple().unwrap();
        let loop_size_card = TransformIterator::new(7).take_while(|v| *v != pub_card).count();
        let encryption_key = TransformIterator::new(pub_door).nth(loop_size_card).unwrap();
        encryption_key.to_result()
    }

    fn solve_version02(&self, _input: ProblemInput, _is_sample: bool) -> ProblemResult {
        ProblemResult::NoPartTwo
    }
}
