use core::panic;
use std::{collections::HashMap, iter};

use aoc_lib::iterator::ParsedExt;
use itertools::Itertools;

use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut c = Cache::new();
    parse(input)
        .into_iter()
        .map(|(line, groups)| find_arrangements(&line, &groups, None, &mut c))
        .sum::<usize>()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    // Takes a couple of seconds for real input
    let mut c = Cache::new();
    parse2(input)
        .into_iter()
        .map(|(line, groups)| find_arrangements(&line, &groups, None, &mut c))
        .sum::<usize>()
        .into()
}

type Cache = HashMap<(String, Vec<usize>, Option<char>), usize>;

fn parse(input: ProblemInput) -> Vec<(String, Vec<usize>)> {
    input
        .lines()
        .into_iter()
        .map(|row| {
            let (line, groups_str) = row.split_once(' ').unwrap();
            let groups = groups_str.split(',').parsed().collect_vec();
            (String::from(line), groups)
        })
        .collect_vec()
}

fn parse2(input: ProblemInput) -> Vec<(String, Vec<usize>)> {
    let base_lines = parse(input);
    base_lines
        .into_iter()
        .map(|(s, g)| {
            (
                Itertools::intersperse(iter::repeat_n(s.chars(), 5), "?".chars())
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
        '.' => match (groups, last_char) {
            (_, Some('.')) | (_, None) => find_arrangements(&line[1..], groups, Some('.'), cache),
            ([0, rest @ ..], Some('#')) => find_arrangements(&line[1..], rest, Some('.'), cache),
            ([_, ..], Some('#')) => 0,
            _ => panic!("what is this"),
        },
        '#' => match groups {
            [] => 0,
            [0, ..] => 0,
            [_, ..] => find_arrangements(
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
            find_arrangements(&format!(".{0}", &line[1..]), groups, last_char, cache)
                + find_arrangements(&format!("#{0}", &line[1..]), groups, last_char, cache)
        }
        _ => panic!("Unexpected char"),
    };

    cache.insert((String::from(line), groups.to_vec(), last_char), arrangements);
    arrangements
}
