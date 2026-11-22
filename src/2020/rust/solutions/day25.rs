use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let (pub_door, pub_card) = input.lines().into_iter().parsed::<u64>().collect_tuple().unwrap();
    let loop_size_card = TransformIterator::new(7).take_while(|v| *v != pub_card).count();
    let encryption_key = TransformIterator::new(pub_door).nth(loop_size_card).unwrap();
    encryption_key.into()
}

pub fn part2(_input: ProblemInput, _is_sample: bool) -> ProblemResult {
    ProblemResult::NoPartTwo
}

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
