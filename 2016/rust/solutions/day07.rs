use std::collections::HashSet;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution07;
impl Solution07 {
    fn supports_tls(ip: &str) -> bool {
        let mut bracket = 0;
        let mut abba = Vec::new();
        let mut has_abba = false;
        for c in ip.chars() {
            match c {
                '[' => {
                    bracket += 1;
                    abba.clear();
                }
                ']' => {
                    bracket -= 1;
                    abba.clear();
                }
                _ => {
                    abba = match &abba[..] {
                        [] => vec![c],
                        [a] if c != *a => vec![*a, c],
                        [a, b] if c == *b => vec![*a, *b, *b],
                        [_, a] if c != *a => vec![*a, c],
                        [a, _, _] if c == *a && bracket != 0 => return false,
                        [a, _, _] if c == *a && bracket == 0 => {
                            has_abba = true;
                            vec![c]
                        }
                        [_, _, a] if c != *a => vec![*a, c],
                        _ => vec![c],
                    }
                }
            }
        }

        has_abba
    }

    fn supports_ssl(ip: &str) -> bool {
        let mut bracket = 0;
        let (mut abas, mut babs) = (HashSet::new(), HashSet::new());
        let mut aba = Vec::new();
        for c in ip.chars() {
            match c {
                '[' => {
                    bracket += 1;
                    aba.clear();
                }
                ']' => {
                    bracket -= 1;
                    aba.clear();
                }
                _ => {
                    aba = match &aba[..] {
                        [] => vec![c],
                        [a] if c != *a => vec![*a, c],
                        [a, b] if c == *a => {
                            if bracket == 0 {
                                abas.insert((*a, *b));
                            } else {
                                babs.insert((*a, *b));
                            }
                            vec![*b, *a]
                        }
                        [_, a] if c != *a => vec![*a, c],
                        _ => vec![c],
                    };
                }
            }
        }

        abas.into_iter().any(|(a, b)| babs.contains(&(b, a)))
    }
}

impl Solution for Solution07 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        input
            .lines()
            .into_iter()
            .filter(|ip| Self::supports_tls(ip))
            .count()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        input
            .lines()
            .into_iter()
            .filter(|ip| Self::supports_ssl(ip))
            .count()
            .to_result()
    }
}
