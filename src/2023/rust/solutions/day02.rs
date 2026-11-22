use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;
use std::collections::HashMap;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    parse(input)
        .iter()
        .filter(|(_, reveals)| is_valid(reveals, 12, 13, 14))
        .map(|(g, _)| g)
        .sum::<u16>()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    parse(input)
        .iter()
        .map(|(_, reveals)| power(reveals))
        .sum::<i32>()
        .into()
}

type Reveals = Vec<(u16, String)>;

fn parse(input: ProblemInput) -> Vec<(u16, Reveals)> {
    let mut record = vec![];
    for line in input.lines() {
        let line_parts = line.split([':', ';', ',']).collect_vec();
        let (game_id_str, reveal_strs) = line_parts.split_first().unwrap();
        let game_id = game_id_str.split_whitespace().nth(1).unwrap().parse::<u16>().unwrap();
        let reveals = reveal_strs
            .iter()
            .map(|&r| r.split_whitespace().take(2).collect_tuple().unwrap())
            .map(|(num, col)| (num.parse::<u16>().unwrap(), String::from(col)))
            .collect::<Vec<_>>();
        record.push((game_id, reveals));
    }
    record
}

fn is_valid(reveals: &Reveals, reds: u16, greens: u16, blues: u16) -> bool {
    let colors = HashMap::from([("red", reds), ("green", greens), ("blue", blues)]);
    reveals.iter().all(|(n, color)| n <= colors.get(color as &str).unwrap())
}

fn power(reveals: &Reveals) -> i32 {
    let mut setsize = HashMap::from([("red", 0u16), ("green", 0), ("blue", 0)]);
    for (n, color) in reveals {
        setsize.entry(color).and_modify(|val| *val = (*val).max(*n));
    }

    setsize.values().product::<u16>().into()
}
