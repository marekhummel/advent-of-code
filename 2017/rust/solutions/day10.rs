use std::sync::Arc;
use std::usize;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

struct KnotHash {
    n: usize,
    lengths: Vec<usize>,
    circle: Vec<usize>,
    i: usize,
    skip: usize,
}

impl KnotHash {
    fn new(n: usize, lengths: Vec<usize>) -> Self {
        KnotHash {
            n,
            lengths,
            circle: (0..n).collect_vec(),
            i: 0,
            skip: 0,
        }
    }

    fn round(&mut self) {
        for length in self.lengths.iter() {
            let mut doubled = [self.circle.clone(), self.circle.clone()].concat();
            let reversed = doubled.iter().skip(self.i).take(*length).copied().rev().collect_vec();
            doubled.splice(self.i..self.i + length, reversed);
            self.circle = [&doubled[self.n..self.n + self.i], &doubled[self.i..self.n]].concat();

            self.i = (self.i + length + self.skip) % self.n;
            self.skip += 1;
        }
    }

    fn to_dense(&self) -> Vec<usize> {
        self.circle
            .iter()
            .chunks(16)
            .into_iter()
            .map(|block| block.fold(0, |acc, elem| acc ^ elem))
            .collect_vec()
    }
}

pub struct Solution10;
impl Solution10 {}

impl Solution for Solution10 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let n = if _is_sample { 5 } else { 256 };
        let lengths = input.string().split(',').parsed::<usize>().collect_vec();

        let mut knot = KnotHash::new(n, lengths);
        knot.round();

        (knot.circle[0] * knot.circle[1]).into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let n = 256;
        let lengths = input.string().bytes().chain([17, 31, 73, 47, 23]).collect_vec();

        let mut knot = KnotHash::new(n, lengths.into_iter().map(|l| l as usize).collect_vec());
        for _ in 0..64 {
            knot.round();
        }

        let dense = knot.to_dense();
        let hash = dense.into_iter().map(|d| format!("{d:02x}")).join("");
        hash.into_some()
    }
}
