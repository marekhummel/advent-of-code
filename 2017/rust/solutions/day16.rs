use std::collections::HashMap;

use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

pub struct Solution16;
impl Solution16 {
    fn parse(input: ProblemInput) -> Vec<Move> {
        input
            .string()
            .split(',')
            .map(|m| {
                let (mt, margs) = m.split_at(1);
                match mt {
                    "s" => Move::Spin(margs.parse().unwrap()),
                    "x" => {
                        let (a, b) = margs.split_once('/').unwrap();
                        Move::Exchange(a.parse().unwrap(), b.parse().unwrap())
                    }
                    "p" => {
                        let (a, b) = margs.split_once('/').unwrap();
                        Move::Partner(a.chars().next().unwrap(), b.chars().next().unwrap())
                    }
                    _ => unreachable!(),
                }
            })
            .collect()
    }

    fn dance(formation: &str, moves: &[Move]) -> String {
        let mut programs = formation.chars().collect_vec();
        for m in moves {
            match m {
                Move::Spin(x) => {
                    programs = [&programs[programs.len() - x..], &programs[0..programs.len() - x]].concat()
                }
                Move::Exchange(a, b) => programs.swap(*a, *b),
                Move::Partner(a, b) => {
                    let x = programs.iter().position(|p| p == a).unwrap();
                    let y = programs.iter().position(|p| p == b).unwrap();
                    programs.swap(x, y);
                }
            };
        }

        programs.into_iter().join("")
    }
}

impl Solution for Solution16 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let moves = Self::parse(input);

        let n = if is_sample { 5 } else { 16 };
        let programs = (0..n).map(|i| (b'a' + i) as char).join("");
        Self::dance(&programs, &moves).into_some()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        if is_sample {
            return None;
        }

        let moves = Self::parse(input);

        let mut programs = (0..16).map(|i| (b'a' + i) as char).join("");

        let mut seen = HashMap::new();
        let mut counter = 0;
        while !seen.contains_key(&programs) {
            seen.insert(programs.clone(), counter);
            programs = Self::dance(&programs, &moves);
            counter += 1;
        }

        let last_counter = seen[&programs];
        let period = counter - last_counter;
        let offset = (1_000_000_000 - last_counter) % period;
        let formation = (0..offset).fold(programs, |form, _| Self::dance(&form, &moves));

        formation.into_some()
    }
}
