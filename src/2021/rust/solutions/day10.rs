use std::collections::HashMap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution10;
impl Solution10 {
    fn fix_subsystem(lines: &[String]) -> (u64, u64) {
        let corruption_table = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
        let autocomplete_table = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

        let mut corruption_score = 0;
        let mut autocomplete_scores = Vec::new();
        for line in lines {
            let mut corrupted = None;
            let mut stack = Vec::new();
            for c in line.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    ')' | ']' | '}' | '>' => {
                        if let Some(pc) = stack.pop() {
                            if pc == c {
                                continue;
                            }
                        }

                        corrupted = Some(c);
                        break;
                    }
                    _ => unreachable!(),
                }
            }

            if let Some(illegal) = corrupted {
                // Corrupted char found
                corruption_score += corruption_table[&illegal];
            } else if !stack.is_empty() {
                // Not corrupted, but stack not empty thus incomplete
                let score = stack.iter().rev().fold(0, |acc, c| acc * 5 + autocomplete_table[c]);
                autocomplete_scores.push(score);
            }
        }

        autocomplete_scores.sort();
        (corruption_score, autocomplete_scores[autocomplete_scores.len() / 2])
    }
}

impl Solution for Solution10 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U64(26397),
            ProblemResult::U64(319329),
            ProblemResult::U64(288957),
            ProblemResult::U64(3515583998),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        Self::fix_subsystem(&input.lines()).0.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        Self::fix_subsystem(&input.lines()).1.to_result()
    }
}
