use std::collections::HashMap;

use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;
use regex::Regex;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let table = parse(input);
    optimal_happiness(&table).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut table = parse(input);
    let me = String::from("Me");
    table.iter_mut().for_each(|(_, vals)| _ = vals.insert(me.clone(), 0i16));
    table.insert(me, table.keys().map(|o| (o.clone(), 0i16)).collect());

    // Takes about 4secs
    optimal_happiness(&table).into()
}

fn parse(input: ProblemInput) -> HashMap<String, HashMap<String, i16>> {
    let line_rgx = Regex::new(r"(?P<from>\w*?) would (?P<sign>gain|lose) (?P<units>\d*) happiness units by sitting next to (?P<to>\w+?)\.").unwrap();

    let mut table = HashMap::new();
    for l in input.lines() {
        let captures = line_rgx.captures(&l).unwrap();
        let from = captures.name("from").unwrap().as_str();
        let to = captures.name("to").unwrap().as_str();
        let units = captures.name("units").unwrap().as_str().parse::<i16>().unwrap();
        let sign = match captures.name("sign").unwrap().as_str() {
            "gain" => 1,
            "lose" => -1,
            _ => unreachable!(),
        };

        table
            .entry(from.trim().to_string())
            .or_insert(HashMap::new())
            .insert(to.trim().to_string(), sign * units);
    }

    table
}

fn optimal_happiness(table: &HashMap<String, HashMap<String, i16>>) -> i16 {
    table
        .keys()
        .permutations(table.len())
        .map(|order| {
            order
                .into_iter()
                .circular_tuple_windows()
                .map(|(from, to)| table[from][to] + table[to][from])
                .sum::<i16>()
        })
        .max()
        .unwrap()
}
