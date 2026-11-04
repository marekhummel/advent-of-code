use std::collections::{HashMap, VecDeque};

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Debug)]
enum Yell {
    Operation(String, String, Operation),
    Number(i64),
}

#[derive(Debug, Clone)]
enum Term {
    Op(Box<Term>, Box<Term>, Operation),
    Number(i64),
    Unknown,
}

impl Term {
    fn eval(&self) -> Option<i64> {
        match self {
            Term::Op(ta, tb, op) => match (ta.eval(), tb.eval()) {
                (Some(a), Some(b)) => Some(match op {
                    Operation::Add => a + b,
                    Operation::Sub => a - b,
                    Operation::Mul => a * b,
                    Operation::Div => a / b,
                }),
                _ => None,
            },
            Term::Number(n) => Some(*n),
            Term::Unknown => None,
        }
    }

    fn solve(&self, target: i64) -> i64 {
        match self {
            Term::Op(ea, eb, op) => {
                match (ea.eval(), eb.eval()) {
                    (None, Some(b)) => match op {
                        Operation::Add => ea.solve(target - b),
                        Operation::Sub => ea.solve(target + b),
                        Operation::Mul => ea.solve(target / b),
                        Operation::Div => ea.solve(target * b),
                    },
                    (Some(a), None) => match op {
                        Operation::Add => eb.solve(target - a),
                        Operation::Sub => eb.solve(-(target - a)),
                        Operation::Mul => eb.solve(target / a),
                        Operation::Div => eb.solve(1 / (target / a)),
                    },
                    (Some(_), Some(_)) => unreachable!(), // no unknowns, nothing to solve
                    (None, None) => unreachable!(),       // two unknowns, can't solve
                }
            }
            Term::Unknown => target,
            Term::Number(_) => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Solution21;
impl Solution21 {
    fn parse(input: ProblemInput) -> Vec<(String, Yell)> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (monkey, message) = l.split_once(": ").unwrap();
                let yell = match message.parse() {
                    Ok(num) => Yell::Number(num),
                    _ => {
                        let (a, op_str, b) = message.split_ascii_whitespace().collect_tuple().unwrap();
                        let op = match op_str {
                            "+" => Operation::Add,
                            "-" => Operation::Sub,
                            "*" => Operation::Mul,
                            "/" => Operation::Div,
                            _ => unreachable!(),
                        };
                        Yell::Operation(a.to_string(), b.to_string(), op)
                    }
                };
                (monkey.to_string(), yell)
            })
            .collect()
    }

    fn get_root(mut monkeys: VecDeque<(String, Yell)>, with_human: bool) -> Term {
        let mut yelled = HashMap::new();
        while let Some((monkey, yell)) = monkeys.pop_front() {
            match yell {
                // Insert unknown if part two and humn monkey
                Yell::Number(_) if with_human && monkey == "humn" => _ = yelled.insert(monkey, Term::Unknown),
                // Insert number
                Yell::Number(num) => _ = yelled.insert(monkey, Term::Number(num)),
                // Try to eval operation
                Yell::Operation(ref ma, ref mb, op) => match (yelled.get(ma), yelled.get(mb)) {
                    // Both operands are evaluated, thus this can be evaluated too
                    (Some(ea), Some(eb)) => {
                        let result = Term::Op(Box::new(ea.clone()), Box::new(eb.clone()), op);

                        // Root monkey, return the operation
                        if monkey == "root" {
                            return result;
                        }
                        yelled.insert(monkey, result);
                    }
                    _ => monkeys.push_back((monkey, yell)),
                },
            }
        }

        unreachable!()
    }
}

impl Solution for Solution21 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I64(152),
            ProblemResult::I64(121868120894282),
            ProblemResult::I64(301),
            ProblemResult::I64(3582317956029),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let monkeys: VecDeque<_> = Self::parse(input).into_iter().collect();
        let root = Self::get_root(monkeys, false);
        root.eval().unwrap().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let monkeys: VecDeque<_> = Self::parse(input).into_iter().collect();
        let Term::Op(a, b, _) = Self::get_root(monkeys, true) else { unreachable!() };

        // Solve equation, left side contains unknown "humn"
        assert!(a.eval().is_none());
        let target = b.eval().unwrap();
        a.solve(target).to_result()
    }
}
