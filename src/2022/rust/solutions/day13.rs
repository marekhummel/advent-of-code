use std::collections::VecDeque;
use std::fmt::Display;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use regex::Regex;

#[derive(Clone)]
enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(tokens: &mut VecDeque<String>) -> Self {
        let next = tokens.pop_front().unwrap();

        if let Ok(num) = next.parse() {
            return Self::Int(num);
        }

        assert_eq!(next, "[");
        if tokens[0] == "]" {
            _ = tokens.pop_front();
            return Self::List(vec![]);
        }

        let mut children = Vec::new();
        loop {
            children.push(Self::parse(tokens));
            if tokens.pop_front().unwrap() == "]" {
                break;
            }
        }
        Self::List(children)
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Int(n) => write!(f, "{}", n)?,
            Packet::List(l) => {
                write!(f, "[")?;
                for p in l {
                    write!(f, "{},", p)?;
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => left.cmp(right),
            (Self::List(left), Self::List(right)) => left.cmp(right),
            (Self::Int(left), right @ Self::List(_)) => Self::List(vec![Self::Int(*left)]).cmp(right),
            (left @ Self::List(_), right @ Self::Int(_)) => right.cmp(left).reverse(),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => left == right,
            (Self::List(left), Self::List(right)) => left == right,
            (Self::Int(left), right @ Self::List(_)) => &Self::List(vec![Self::Int(*left)]) == right,
            (left @ Self::List(_), right @ Self::Int(_)) => right == left,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Packet {}

pub struct Solution13;
impl Solution13 {
    fn parse(input: ProblemInput) -> Vec<Packet> {
        let token_rgx = Regex::new(r"(\[|\]|,|\d+)").unwrap();

        input
            .lines()
            .into_iter()
            .filter(|l| !l.is_empty())
            .map(|p| {
                let mut tokens = token_rgx.find_iter(&p).map(|m| m.as_str().to_string()).collect();
                Packet::parse(&mut tokens)
            })
            .collect()
    }
}

impl Solution for Solution13 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(13),
            ProblemResult::USize(5580),
            ProblemResult::USize(140),
            ProblemResult::USize(26200),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let packet_pairs = Self::parse(input).into_iter().tuples();

        let right_order = packet_pairs.enumerate().filter(|(_, (left, right))| left <= right);
        let index_sum = right_order.map(|(i, _)| i + 1).sum::<usize>();

        index_sum.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut packets = Self::parse(input);

        let divider1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
        let divider2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
        packets.extend([divider1.clone(), divider2.clone()]);

        packets.sort();

        let distress1 = packets.iter().position(|p| p == &divider1).unwrap() + 1;
        let distress2 = packets.iter().position(|p| p == &divider2).unwrap() + 1;
        (distress1 * distress2).to_result()
    }
}
