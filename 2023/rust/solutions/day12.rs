use core::panic;
use std::{collections::HashMap, iter};

use itertools::Itertools;

use crate::solution::{ProblemInput, ProblemResult, Solution};

type Cache = HashMap<(String, Vec<usize>, Option<char>), usize>;

pub struct Solution12;
impl Solution12 {
    fn parse(&self, input: ProblemInput) -> Vec<(String, Vec<usize>)> {
        input
            .into_iter()
            .map(|row| {
                let (line, groups_str) = row.split_once(' ').unwrap();
                let groups = groups_str.split(',').map(|c| c.parse::<usize>().unwrap()).collect_vec();
                (String::from(line), groups)
            })
            .collect_vec()
    }

    fn parse2(&self, input: ProblemInput) -> Vec<(String, Vec<usize>)> {
        let base_lines = self.parse(input);
        base_lines
            .into_iter()
            .map(|(s, g)| {
                (
                    iter::repeat(s.chars())
                        .take(5)
                        .intersperse("?".chars())
                        .flatten()
                        .collect(),
                    g.iter().cycle().take(g.len() * 5).cloned().collect_vec(),
                )
            })
            .collect_vec()
    }

    fn find_arrangements(line: &str, groups: &[usize], last_char: Option<char>, cache: &mut Cache) -> usize {
        if let Some(cached) = cache.get(&(String::from(line), groups.to_vec(), last_char)) {
            return *cached;
        }

        if line.is_empty() {
            match groups {
                [] => return 1,
                [0] => return 1,
                _ => return 0,
            }
        }

        let arrangements = match line.chars().next().unwrap() {
            '.' => {
                let new_groups_opt = match (groups, last_char) {
                    (_, Some('.')) | (_, None) => Some(groups),
                    ([0, rest @ ..], Some('#')) => Some(rest),
                    ([_, ..], Some('#')) => None,
                    _ => panic!("what is this"),
                };
                match new_groups_opt {
                    Some(new_groups) => Self::find_arrangements(&line[1..], new_groups, Some('.'), cache),
                    None => 0,
                }
            }
            '#' => match groups {
                [] => 0,
                [0, ..] => 0,
                [_, ..] => Self::find_arrangements(
                    &line[1..],
                    &groups
                        .iter()
                        .enumerate()
                        .map(|(i, v)| v - (i == 0) as usize)
                        .collect_vec(),
                    Some('#'),
                    cache,
                ),
            },
            '?' => {
                let use_dot = format!(".{0}", &line[1..]);
                let use_hash = format!("#{0}", &line[1..]);
                Self::find_arrangements(&use_dot, groups, last_char, cache)
                    + Self::find_arrangements(&use_hash, groups, last_char, cache)
            }
            _ => panic!("Unexpected char"),
        };

        cache.insert((String::from(line), groups.to_vec(), last_char), arrangements);
        arrangements
    }
}

impl Solution for Solution12 {
    fn get_day(&self) -> u8 {
        12
    }

    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        let mut c = Cache::new();
        Some(
            self.parse(input)
                .into_iter()
                .map(|(line, groups)| Self::find_arrangements(&line, &groups, None, &mut c))
                .sum::<usize>()
                .into(),
        )
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        let mut c = Cache::new();
        Some(
            self.parse2(input)
                .into_iter()
                .map(|(line, groups)| Self::find_arrangements(&line, &groups, None, &mut c))
                .sum::<usize>()
                .into(),
        )
    }
}
