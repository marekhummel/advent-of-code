use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult};
use itertools::Itertools;
use std::collections::HashMap;

type Reveals = Vec<(u16, String)>;

pub struct Solution02;

impl Solution02 {
    fn parse(&self, input: ProblemInput) -> Vec<(u16, Reveals)> {
        let mut record = vec![];
        for line in input {
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

    fn is_valid(&self, reveals: &Reveals, reds: u16, greens: u16, blues: u16) -> bool {
        let colors = HashMap::from([("red", reds), ("green", greens), ("blue", blues)]);
        reveals.iter().all(|(n, color)| n <= colors.get(color as &str).unwrap())
    }

    fn power(&self, reveals: &Reveals) -> i32 {
        let mut setsize = HashMap::from([("red", 0u16), ("green", 0), ("blue", 0)]);
        for (n, color) in reveals {
            setsize.entry(color).and_modify(|val| *val = (*val).max(*n));
        }

        setsize.values().product::<u16>().into()
    }
}

impl Solution for Solution02 {
    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        Some(
            self.parse(input)
                .iter()
                .filter(|(_, reveals)| self.is_valid(reveals, 12, 13, 14))
                .map(|(g, _)| g)
                .sum::<u16>()
                .into(),
        )
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        Some(
            self.parse(input)
                .iter()
                .map(|(_, reveals)| self.power(reveals))
                .sum::<i32>()
                .into(),
        )
    }
}
