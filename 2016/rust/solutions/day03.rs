use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution03;
impl Solution03 {
    fn parse(input: ProblemInput) -> Vec<(u16, u16, u16)> {
        input
            .lines()
            .into_iter()
            .map(|l| l.split_whitespace().parsed().collect_tuple().unwrap())
            .collect_vec()
    }

    fn parse2(input: ProblemInput) -> Vec<(u16, u16, u16)> {
        input
            .lines()
            .into_iter()
            .chunks(3)
            .into_iter()
            .flat_map(|row_chunk| {
                let ns = row_chunk
                    .flat_map(|l| l.split_whitespace().parsed().collect_vec())
                    .enumerate()
                    .collect_vec();

                (0..3)
                    .map(|r| {
                        ns.iter()
                            .filter(|(i, _)| i % 3 == r)
                            .map(|(_, n)| *n)
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect_vec()
            })
            .collect_vec()
    }

    fn is_triangle(triangle: &(u16, u16, u16)) -> bool {
        let (a, b, c) = *triangle;
        a + b > c && a + c > b && b + c > a
    }
}

impl Solution for Solution03 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let triangles = Self::parse(input);
        triangles.into_iter().filter(Self::is_triangle).count().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let triangles = Self::parse2(input);
        triangles.into_iter().filter(Self::is_triangle).count().to_result()
    }
}
