use std::collections::HashMap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Number(u64),
    Op(Operator),
    OpenParen,
    CloseParen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
}

pub struct Solution18;
impl Solution18 {
    fn eval(expression: &str, precedence: &HashMap<Operator, u8>) -> u64 {
        let tokens = Self::tokenize(expression);
        let postfix = Self::shunting_yard(tokens, precedence);

        let mut result = Vec::new();
        for token in postfix {
            match token {
                Token::Number(num) => result.push(num),
                Token::Op(op) => {
                    let arg2 = result.pop().unwrap();
                    let arg1 = result.pop().unwrap();

                    match op {
                        Operator::Add => result.push(arg1 + arg2),
                        Operator::Multiply => result.push(arg1 * arg2),
                    }
                }
                _ => unreachable!(),
            }
        }

        assert_eq!(result.len(), 1);
        result.pop().unwrap()
    }

    fn tokenize(expression: &str) -> Vec<Token> {
        let regex = Regex::new(r"[+*]|[()]|\d+").unwrap();
        let tokens = regex
            .find_iter(expression)
            .map(|m| match m.as_str() {
                "+" => Token::Op(Operator::Add),
                "*" => Token::Op(Operator::Multiply),
                "(" => Token::OpenParen,
                ")" => Token::CloseParen,
                num => Token::Number(num.parse().unwrap()),
            })
            .collect();
        tokens
    }

    fn shunting_yard(infix: Vec<Token>, precedence: &HashMap<Operator, u8>) -> Vec<Token> {
        let mut stack = Vec::new();
        let mut postfix = Vec::new();

        for curr in infix {
            match curr {
                Token::Number(_) => postfix.push(curr),
                Token::OpenParen => stack.push(curr),
                Token::Op(curr_op) => {
                    // Drain all ops with higher precedence before adding current op
                    let last_op_idx = stack
                        .iter()
                        .rposition(|t| !matches!(t, Token::Op(prev_op) if precedence[&curr_op] <= precedence[&prev_op]))
                        .map(|pos| pos + 1)
                        .unwrap_or(0);
                    let drained = stack.drain(last_op_idx..);
                    postfix.extend(drained.rev());
                    stack.push(curr);
                }
                Token::CloseParen => {
                    // Move all token until last open paren to result
                    let lparen = stack.iter().rposition(|st| *st == Token::OpenParen).unwrap();
                    let drained = stack.drain(lparen..);
                    postfix.extend(drained.skip(1).rev()); // Don't need parentheses in postfix, so skip
                }
            }
        }

        // Add rest of stack
        postfix.extend(stack.iter().rev());

        postfix
    }
}

impl Solution for Solution18 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let precedences = HashMap::from([(Operator::Add, 0), (Operator::Multiply, 0)]);
        let results = input.lines().into_iter().map(|expr| Self::eval(&expr, &precedences));

        results.sum::<u64>().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let precedences = HashMap::from([(Operator::Add, 1), (Operator::Multiply, 0)]);
        let results = input.lines().into_iter().map(|expr| Self::eval(&expr, &precedences));

        results.sum::<u64>().to_result()
    }
}
