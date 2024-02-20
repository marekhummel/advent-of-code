use std::collections::HashMap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::{iproduct, Itertools};

pub struct Solution21;
impl Solution21 {
    fn parse(input: ProblemInput) -> HashMap<String, String> {
        input
            .lines()
            .into_iter()
            .flat_map(|line| {
                let (pattern, outcome_str) = line.split_once("=>").unwrap();
                let outcome: String = outcome_str.trim().split('/').collect();

                // Every pattern has 8 rotations / flips, see dihedral group 4 (symmetry of a square)
                let mut block = pattern.trim().split('/').map(|s| s.chars().collect_vec()).collect_vec();
                let mut rules = Vec::new();
                for _ in 0..2 {
                    for _ in 0..4 {
                        rules.push((
                            block.iter().map(|r| r.iter().collect::<String>()).join(""),
                            outcome.clone(),
                        ));

                        // rotate
                        block = (0..block.len())
                            .rev()
                            .map(|c| (0..block.len()).map(|r| block[r][c]).collect_vec())
                            .collect_vec()
                    }

                    // flip
                    block = block.into_iter().map(|r| r.into_iter().rev().collect()).collect();
                }

                rules
            })
            .collect()
    }

    fn enhance(canvas: Vec<Vec<char>>, rules: &HashMap<String, String>) -> Vec<Vec<char>> {
        let size = match (canvas.len() % 2, canvas.len() % 3) {
            (0, _) => 2,
            (_, 0) => 3,
            _ => unreachable!(),
        };

        let blocks = canvas.len() / size;
        let nsize = size + 1;
        let mut new_canvas = vec![vec![' '; blocks * nsize]; blocks * nsize];
        for bj in 0..blocks {
            for bi in 0..blocks {
                let block = iproduct!((bj * size..(bj + 1) * size), (bi * size..(bi + 1) * size));
                let pattern = block.map(|(j, i)| canvas[j][i]).collect::<String>();

                let enhancement = rules[&pattern].chars();
                let new_block = iproduct!((bj * nsize..(bj + 1) * nsize), (bi * nsize..(bi + 1) * nsize));
                for (c, (j, i)) in enhancement.zip_eq(new_block) {
                    new_canvas[j][i] = c;
                }
            }
        }

        new_canvas
    }

    fn count_leds(canvas: &[Vec<char>]) -> usize {
        canvas
            .iter()
            .map(|block| block.iter().filter(|c| **c == '#').count())
            .sum()
    }
}

impl Solution for Solution21 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let rules = Self::parse(input);
        let iterations = if is_sample { 2 } else { 5 };
        let mut canvas = vec![vec!['.', '#', '.'], vec!['.', '.', '#'], vec!['#', '#', '#']];

        for _ in 0..iterations {
            canvas = Self::enhance(canvas, &rules);
        }

        Self::count_leds(&canvas).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let rules = Self::parse(input);
        let iterations = if is_sample { 2 } else { 18 };
        let mut canvas = vec![vec!['.', '#', '.'], vec!['.', '.', '#'], vec!['#', '#', '#']];

        for _ in 0..iterations {
            canvas = Self::enhance(canvas, &rules);
        }

        Self::count_leds(&canvas).to_result()
    }
}
