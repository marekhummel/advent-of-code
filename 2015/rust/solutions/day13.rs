use std::collections::HashMap;

use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;
use regex::Regex;

pub struct Solution13;
impl Solution13 {
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
}

impl Solution for Solution13 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let table = Self::parse(input);
        Self::optimal_happiness(&table).into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut table = Self::parse(input);
        let me = String::from("Me");
        table.iter_mut().for_each(|(_, vals)| _ = vals.insert(me.clone(), 0i16));
        table.insert(me.clone(), table.keys().map(|o| (o.clone(), 0i16)).collect());

        // Takes about 4secs
        Self::optimal_happiness(&table).into_some()
    }
}
